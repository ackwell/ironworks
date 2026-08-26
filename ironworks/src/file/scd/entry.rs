use std::io::{Cursor, Seek, SeekFrom};

use binrw::{BinRead, binread};
use derivative::Derivative;
use getset::{CopyGetters, Getters};

use crate::error::{Error, ErrorValue, Result};

/// A single audio stream within a [`SoundContainer`](super::SoundContainer).
#[derive(Derivative, Getters, CopyGetters, Clone)]
#[derivative(Debug)]
pub struct SoundEntry {
	/// Codec the audio stream is encoded with.
	#[get_copy = "pub"]
	format: Codec,

	/// Number of channels.
	#[get_copy = "pub"]
	channel_count: u32,

	/// Sample rate in Hz.
	#[get_copy = "pub"]
	sample_rate: u32,

	/// Loop start, as a byte offset into the compressed audio body (not a sample index).
	#[get_copy = "pub"]
	loop_start: u32,

	/// Loop end, as a byte offset into the compressed audio body; `0` means no loop.
	#[get_copy = "pub"]
	loop_end: u32,

	/// Sample positions from the stream's `MARK` chunk, if it carries one.
	#[get = "pub"]
	markers: Vec<u32>,

	/// Bytes per encoded block, for [`Codec::MsAdpcm`] only: needed to translate `loop_start`/
	/// `loop_end` (byte offsets) into sample frames.
	#[get_copy = "pub"]
	adpcm_block_align: Option<u16>,

	/// Samples per encoded block, for [`Codec::MsAdpcm`] only; see [`Self::adpcm_block_align`].
	#[get_copy = "pub"]
	adpcm_samples_per_block: Option<u16>,

	/// Decode-ready audio: a standalone `.ogg` for [`Codec::OggVorbis`], a standalone `.hca` for
	/// [`Codec::Hca`], a standalone WAVE_FORMAT_ADPCM `.wav` for [`Codec::MsAdpcm`], else the raw
	/// payload.
	#[derivative(Debug = "ignore")]
	#[get = "pub"]
	data: Vec<u8>,
}

impl SoundEntry {
	pub(super) fn parse(bytes: &[u8], offset: usize) -> Result<Self> {
		let mut cursor = Cursor::new(bytes);
		cursor.seek(SeekFrom::Start(offset as u64))?;
		let desc = AudioBasicDesc::read(&mut cursor)?;

		let sub = offset
			.checked_add(AUDIO_DESC_SIZE)
			.ok_or_else(|| invalid("audio entry offset overflows"))?;
		let markers = parse_markers(bytes, sub, &desc);

		let format = Codec::from(desc.format);
		let mut adpcm_block_align = None;
		let mut adpcm_samples_per_block = None;
		let data = match format {
			Codec::Empty => Vec::new(),
			Codec::OggVorbis => descramble_ogg(bytes, sub, &desc)?,
			Codec::Hca => extract_hca(bytes, sub, &desc)?,
			Codec::MsAdpcm => {
				let adpcm = build_adpcm_wav(bytes, sub, &desc)?;
				adpcm_block_align = Some(adpcm.block_align);
				adpcm_samples_per_block = Some(adpcm.samples_per_block);
				adpcm.wav
			}
			_ => {
				let start = sub
					.checked_add(desc.sub_info_size as usize)
					.ok_or_else(|| invalid("audio entry sub-info size overflows"))?;
				slice(bytes, start, desc.data_size as usize)?.to_vec()
			}
		};

		Ok(Self {
			format,
			channel_count: desc.channel_count,
			sample_rate: desc.sample_rate,
			loop_start: desc.loop_start,
			loop_end: desc.loop_end,
			markers,
			adpcm_block_align,
			adpcm_samples_per_block,
			data,
		})
	}
}

const AUDIO_DESC_SIZE: usize = 32;

#[binread]
#[br(little)]
#[derive(Debug)]
struct AudioBasicDesc {
	data_size: u32,
	channel_count: u32,
	sample_rate: u32,
	format: i32,
	loop_start: u32,
	loop_end: u32,
	sub_info_size: u32,
	aux_flags: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct OggHeader {
	version: u8,
	#[br(pad_before = 1)]
	xor_byte: u8,
	#[br(pad_before = 9 + 4 + 4)]
	ogg_header_size: u32,
}

/// The codec an audio stream is encoded with.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
	Empty,
	Pcm,
	OggVorbis,
	Mp3,
	MsAdpcm,
	Atrac9,
	Hca,
	/// A codec ironworks does not recognise; the inner value is the raw format tag.
	Unknown(i32),
}

impl From<i32> for Codec {
	fn from(value: i32) -> Self {
		match value {
			-1 => Self::Empty,
			0x1 => Self::Pcm,
			0x6 => Self::OggVorbis,
			0x7 => Self::Mp3,
			0xC => Self::MsAdpcm,
			0x16 => Self::Atrac9,
			0x1A => Self::Hca,
			other => Self::Unknown(other),
		}
	}
}

fn descramble_ogg(bytes: &[u8], sub: usize, desc: &AudioBasicDesc) -> Result<Vec<u8>> {
	let mut cursor = Cursor::new(bytes);
	// A leading marker chunk (aux_flags bit 0) precedes the ogg header pages; its size is its
	// own second word, so skip it by that rather than by re-parsing it here.
	let marker_len = if desc.aux_flags & 1 != 0 {
		cursor.seek(SeekFrom::Start(
			sub.checked_add(4)
				.ok_or_else(|| invalid("marker offset overflows"))? as u64,
		))?;
		u32::read_le(&mut cursor)? as usize
	} else {
		0
	};

	let header_start = sub
		.checked_add(marker_len)
		.ok_or_else(|| invalid("marker length overflows"))?;
	cursor.seek(SeekFrom::Start(header_start as u64))?;
	let header = OggHeader::read(&mut cursor)?;
	let header_size = header.ogg_header_size as usize;
	let data_size = desc.data_size as usize;

	let start = sub
		.checked_add(desc.sub_info_size as usize)
		.and_then(|end| end.checked_sub(header_size))
		.ok_or_else(|| invalid("ogg header size exceeds sub-info region"))?;
	let total = header_size
		.checked_add(data_size)
		.ok_or_else(|| invalid("ogg stream size overflows"))?;
	let mut ogg = slice(bytes, start, total)?.to_vec();

	match header.version {
		2 => {
			for byte in &mut ogg[..header_size] {
				*byte ^= header.xor_byte;
			}
		}
		3 => xor_v3(&mut ogg, desc.data_size, 0),
		_ => {}
	}

	Ok(ogg)
}

/// Reconstruct a standalone, decodable HCA from an HCA stream.
fn extract_hca(bytes: &[u8], sub: usize, desc: &AudioBasicDesc) -> Result<Vec<u8>> {
	let sub_info_size = desc.sub_info_size as usize;
	let sub_info = slice(bytes, sub, sub_info_size)?;
	let header_pos =
		find_hca_magic(sub_info).ok_or_else(|| invalid("no HCA header in sub-info"))?;

	let header_start = sub
		.checked_add(header_pos)
		.ok_or_else(|| invalid("hca header offset overflows"))?;
	let header = slice(bytes, header_start, sub_info_size - header_pos)?;
	let frames_start = sub
		.checked_add(sub_info_size)
		.ok_or_else(|| invalid("hca frame offset overflows"))?;
	let frames = slice(bytes, frames_start, desc.data_size as usize)?;

	let mut hca = Vec::with_capacity(header.len() + frames.len());
	hca.extend_from_slice(header);
	hca.extend_from_slice(frames);

	// Descramble the frames unless they are already plaintext (frame sync `0xFFFF`).
	let frame_bytes = &mut hca[header.len()..];
	if !frame_bytes.starts_with(&[0xFF, 0xFF]) {
		xor_v3(frame_bytes, desc.data_size, header.len());
	}

	Ok(hca)
}

struct AdpcmWav {
	wav: Vec<u8>,
	block_align: u16,
	samples_per_block: u16,
}

/// Reconstruct a standalone WAVE_FORMAT_ADPCM `.wav` from an MS ADPCM stream: the sub-info is
/// already a WAVEFORMATEX (base fields plus the ADPCM coefficient table) once any marker chunk in
/// front of it is skipped, so it drops straight into a `fmt ` chunk unparsed.
fn build_adpcm_wav(bytes: &[u8], sub: usize, desc: &AudioBasicDesc) -> Result<AdpcmWav> {
	let mut cursor = Cursor::new(bytes);
	let marker_len = if desc.aux_flags & 1 != 0 {
		cursor.seek(SeekFrom::Start(
			sub.checked_add(4)
				.ok_or_else(|| invalid("marker offset overflows"))? as u64,
		))?;
		u32::read_le(&mut cursor)? as usize
	} else {
		0
	};

	let fmt_start = sub
		.checked_add(marker_len)
		.ok_or_else(|| invalid("marker length overflows"))?;
	// cbSize sits right after the 16-byte WAVEFORMATEX prefix (tag, channels, rate, byte rate,
	// block align, bits per sample); the ADPCM coefficient table follows it in turn.
	let cb_size_at = fmt_start
		.checked_add(16)
		.ok_or_else(|| invalid("adpcm header offset overflows"))?;
	let cb_size = u16::from_le_bytes(slice(bytes, cb_size_at, 2)?.try_into().unwrap());
	let fmt_len = 18usize
		.checked_add(cb_size as usize)
		.ok_or_else(|| invalid("adpcm format size overflows"))?;
	let fmt_chunk = slice(bytes, fmt_start, fmt_len)?;
	if fmt_chunk.len() < 20 {
		return Err(invalid("adpcm format chunk too short"));
	}

	let format_tag = u16::from_le_bytes(fmt_chunk[0..2].try_into().unwrap());
	if format_tag != 2 {
		return Err(invalid("adpcm sub-info is not WAVE_FORMAT_ADPCM"));
	}
	let block_align = u16::from_le_bytes(fmt_chunk[12..14].try_into().unwrap());
	let samples_per_block = u16::from_le_bytes(fmt_chunk[18..20].try_into().unwrap());

	let data_start = sub
		.checked_add(desc.sub_info_size as usize)
		.ok_or_else(|| invalid("audio entry sub-info size overflows"))?;
	let data = slice(bytes, data_start, desc.data_size as usize)?;

	Ok(AdpcmWav {
		wav: wrap_wav(fmt_chunk, data)?,
		block_align,
		samples_per_block,
	})
}

/// A minimal RIFF/WAVE container: `fmt ` then `data`, each padded to an even length as RIFF
/// requires.
fn wrap_wav(fmt_chunk: &[u8], data: &[u8]) -> Result<Vec<u8>> {
	let chunk_len = |len: usize| -> Result<u32> {
		u32::try_from(len).map_err(|_| invalid("wav chunk size overflows"))
	};
	let padded = |len: usize| -> Result<usize> {
		len.checked_add(len % 2)
			.ok_or_else(|| invalid("wav chunk padding overflows"))
	};

	let padded_fmt = padded(fmt_chunk.len())?;
	let padded_data = padded(data.len())?;
	let riff_size = 4usize
		.checked_add(8)
		.and_then(|n| n.checked_add(padded_fmt))
		.and_then(|n| n.checked_add(8))
		.and_then(|n| n.checked_add(padded_data))
		.ok_or_else(|| invalid("wav container size overflows"))?;

	let mut wav = Vec::with_capacity(8 + riff_size);
	wav.extend_from_slice(b"RIFF");
	wav.extend_from_slice(&chunk_len(riff_size)?.to_le_bytes());
	wav.extend_from_slice(b"WAVE");

	wav.extend_from_slice(b"fmt ");
	wav.extend_from_slice(&chunk_len(fmt_chunk.len())?.to_le_bytes());
	wav.extend_from_slice(fmt_chunk);
	if fmt_chunk.len() % 2 == 1 {
		wav.push(0);
	}

	wav.extend_from_slice(b"data");
	wav.extend_from_slice(&chunk_len(data.len())?.to_le_bytes());
	wav.extend_from_slice(data);
	if data.len() % 2 == 1 {
		wav.push(0);
	}

	Ok(wav)
}

/// A `MARK` chunk in an audio stream's sub-info, present whenever the low aux flag bit is set: tag
/// (4 bytes), size (4), two int32 fields of unproven meaning (8), a record count (4), then the
/// records themselves starting at chunk offset 20. Real records are monotonically ascending sample
/// positions read from 20; reading from 24 instead (an earlier version of this function did)
/// drops the first record and reads one field past the last one present. The chunk's own `size`,
/// not `count`, is what bounds the record list; a few files carry a `count` past what `size`
/// actually reserves.
fn parse_markers(bytes: &[u8], sub: usize, desc: &AudioBasicDesc) -> Vec<u32> {
	if desc.aux_flags & 1 == 0 {
		return Vec::new();
	}
	let Some(region) = bytes
		.get(sub..)
		.and_then(|rest| rest.get(..desc.sub_info_size as usize))
	else {
		return Vec::new();
	};
	if region.get(..4) != Some(b"MARK") {
		return Vec::new();
	}
	let Some(chunk) = region
		.get(4..8)
		.map(le_u32)
		.and_then(|size| region.get(..size as usize))
	else {
		return Vec::new();
	};
	let Some(count) = chunk.get(16..20).map(le_u32) else {
		return Vec::new();
	};
	let available = chunk.len().saturating_sub(20) / 4;
	(0..(count as usize).min(available))
		.map(|index| le_u32(&chunk[20 + index * 4..24 + index * 4]))
		.collect()
}

fn le_u32(bytes: &[u8]) -> u32 {
	u32::from_le_bytes(bytes.try_into().unwrap())
}

/// Find the HCA header in a sub-info region, tolerating `0x80` chunk-tag obfuscation.
fn find_hca_magic(sub_info: &[u8]) -> Option<usize> {
	sub_info
		.windows(4)
		.position(|window| matches!(window, [b'H', b'C', b'A', 0] | [0xC8, 0xC3, 0xC1, 0x80]))
}

/// SqEx v3 XOR descramble. `start_offset` is the position of `buffer[0]` in the stream:
/// `0` for a whole ogg, the header length for HCA frames after a plaintext header.
fn xor_v3(buffer: &mut [u8], data_size: u32, start_offset: usize) {
	let byte1 = (data_size & 0x7F) as u8;
	let byte2 = (data_size & 0x3F) as usize;
	for (index, byte) in buffer.iter_mut().enumerate() {
		*byte ^= OGG_XOR_TABLE[(byte2 + start_offset + index) & 0xFF] ^ byte1;
	}
}

fn slice(bytes: &[u8], start: usize, len: usize) -> Result<&[u8]> {
	let end = start
		.checked_add(len)
		.ok_or_else(|| invalid("audio stream extends past end of file"))?;
	bytes
		.get(start..end)
		.ok_or_else(|| invalid("audio stream extends past end of file"))
}

fn invalid(reason: &str) -> Error {
	Error::Invalid(ErrorValue::Other("SCD".into()), reason.into())
}

/// XOR table for the v3 descramble scheme (OggVorbis and HCA), from Lumina (which is from ffxiv-explorer).
#[rustfmt::skip]
const OGG_XOR_TABLE: [u8; 256] = [
	0x3A,0x32,0x32,0x32,0x03,0x7E,0x12,0xF7,0xB2,0xE2,0xA2,0x67,0x32,0x32,0x22,0x32,
	0x32,0x52,0x16,0x1B,0x3C,0xA1,0x54,0x7B,0x1B,0x97,0xA6,0x93,0x1A,0x4B,0xAA,0xA6,
	0x7A,0x7B,0x1B,0x97,0xA6,0xF7,0x02,0xBB,0xAA,0xA6,0xBB,0xF7,0x2A,0x51,0xBE,0x03,
	0xF4,0x2A,0x51,0xBE,0x03,0xF4,0x2A,0x51,0xBE,0x12,0x06,0x56,0x27,0x32,0x32,0x36,
	0x32,0xB2,0x1A,0x3B,0xBC,0x91,0xD4,0x7B,0x58,0xFC,0x0B,0x55,0x2A,0x15,0xBC,0x40,
	0x92,0x0B,0x5B,0x7C,0x0A,0x95,0x12,0x35,0xB8,0x63,0xD2,0x0B,0x3B,0xF0,0xC7,0x14,
	0x51,0x5C,0x94,0x86,0x94,0x59,0x5C,0xFC,0x1B,0x17,0x3A,0x3F,0x6B,0x37,0x32,0x32,
	0x30,0x32,0x72,0x7A,0x13,0xB7,0x26,0x60,0x7A,0x13,0xB7,0x26,0x50,0xBA,0x13,0xB4,
	0x2A,0x50,0xBA,0x13,0xB5,0x2E,0x40,0xFA,0x13,0x95,0xAE,0x40,0x38,0x18,0x9A,0x92,
	0xB0,0x38,0x00,0xFA,0x12,0xB1,0x7E,0x00,0xDB,0x96,0xA1,0x7C,0x08,0xDB,0x9A,0x91,
	0xBC,0x08,0xD8,0x1A,0x86,0xE2,0x70,0x39,0x1F,0x86,0xE0,0x78,0x7E,0x03,0xE7,0x64,
	0x51,0x9C,0x8F,0x34,0x6F,0x4E,0x41,0xFC,0x0B,0xD5,0xAE,0x41,0xFC,0x0B,0xD5,0xAE,
	0x41,0xFC,0x3B,0x70,0x71,0x64,0x33,0x32,0x12,0x32,0x32,0x36,0x70,0x34,0x2B,0x56,
	0x22,0x70,0x3A,0x13,0xB7,0x26,0x60,0xBA,0x1B,0x94,0xAA,0x40,0x38,0x00,0xFA,0xB2,
	0xE2,0xA2,0x67,0x32,0x32,0x12,0x32,0xB2,0x32,0x32,0x32,0x32,0x75,0xA3,0x26,0x7B,
	0x83,0x26,0xF9,0x83,0x2E,0xFF,0xE3,0x16,0x7D,0xC0,0x1E,0x63,0x21,0x07,0xE3,0x01,
];

#[cfg(test)]
mod test {
	use super::{AudioBasicDesc, parse_markers};

	/// A handful of real files carry a `count` one or two past what the chunk's own `size`
	/// actually reserves; `size` is what has to win.
	#[test]
	fn mark_size_bounds_a_count_that_overstates_it() {
		let mut chunk = Vec::new();
		chunk.extend(b"MARK");
		chunk.extend(28u32.to_le_bytes()); // header (20) + 2 records
		chunk.extend(0u32.to_le_bytes());
		chunk.extend(0u32.to_le_bytes());
		chunk.extend(3u32.to_le_bytes()); // overstates the 2 records the chunk reserves
		chunk.extend(111u32.to_le_bytes());
		chunk.extend(222u32.to_le_bytes());
		chunk.extend(999u32.to_le_bytes()); // past the chunk, must not be read as a record

		let desc = AudioBasicDesc {
			data_size: 0,
			channel_count: 1,
			sample_rate: 44100,
			format: -1,
			loop_start: 0,
			loop_end: 0,
			sub_info_size: chunk.len() as u32,
			aux_flags: 1,
		};
		assert_eq!(parse_markers(&chunk, 0, &desc), vec![111, 222]);
	}

	#[test]
	fn no_marker_bit_reads_no_markers() {
		let desc = AudioBasicDesc {
			data_size: 0,
			channel_count: 1,
			sample_rate: 44100,
			format: -1,
			loop_start: 0,
			loop_end: 0,
			sub_info_size: 0,
			aux_flags: 0,
		};
		assert_eq!(parse_markers(&[], 0, &desc), Vec::<u32>::new());
	}
}
