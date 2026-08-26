use std::io::{Cursor, Seek, SeekFrom};

use binrw::{BinRead, binread};
use derivative::Derivative;

use crate::{FileStream, error::Result, file::File};

use super::entry::SoundEntry;

/// A `.scd` sound container.
#[derive(Debug)]
pub struct SoundContainer {
	entries: Vec<SoundEntry>,
	sound_count: u16,
	track_count: u16,
}

impl SoundContainer {
	/// The audio streams contained in this file.
	pub fn entries(&self) -> &[SoundEntry] {
		&self.entries
	}

	/// The audio stream at `index`, if present.
	pub fn sound(&self, index: usize) -> Option<&SoundEntry> {
		self.entries.get(index)
	}

	/// Sounds declared in the header. Ironworks does not resolve how a sound's tracks map onto
	/// the audio streams in [`entries`](Self::entries); this is only the header's own count.
	pub fn sound_count(&self) -> u16 {
		self.sound_count
	}

	/// Tracks declared in the header, likewise unresolved against `entries`.
	pub fn track_count(&self) -> u16 {
		self.track_count
	}
}

impl File for SoundContainer {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		let mut bytes = Vec::new();
		stream.read_to_end(&mut bytes)?;
		let mut cursor = Cursor::new(&bytes);

		let binary = BinaryHeader::read(&mut cursor)?;
		cursor.seek(SeekFrom::Start(binary.header_offset.into()))?;
		let header = ScdHeader::read(&mut cursor)?;

		cursor.seek(SeekFrom::Start(header.audio_offset.into()))?;
		let offsets = (0..header.audio_count)
			.map(|_| u32::read_le(&mut cursor))
			.collect::<binrw::BinResult<Vec<u32>>>()?;

		let entries = offsets
			.into_iter()
			.enumerate()
			.filter(|&(_, offset)| offset != 0)
			.map(|(slot, offset)| SoundEntry::parse(&bytes, offset as usize, slot as u16))
			.collect::<Result<Vec<_>>>()?;

		Ok(Self {
			entries,
			sound_count: header.sound_count,
			track_count: header.track_count,
		})
	}
}

#[binread]
#[br(little, magic = b"SEDBSSCF")]
#[derive(Derivative)]
#[derivative(Debug)]
struct BinaryHeader {
	version: u32,
	endian: u8,
	align: u8,
	header_offset: u16,
	file_size: u64,
}

#[binread]
#[br(little)]
#[derive(Derivative)]
#[derivative(Debug)]
struct ScdHeader {
	sound_count: u16,
	track_count: u16,
	audio_count: u16,
	number: u16,
	track_offset: u32,
	audio_offset: u32,
	layout_offset: u32,
	routing_offset: u32,
	attribute_offset: u32,
}

#[cfg(test)]
mod test {
	use std::io::Cursor;

	use super::SoundContainer;
	use crate::file::File;

	/// Three audio slots where the middle one is a zero offset (no entry at all, distinct from a
	/// present entry whose codec is `Empty`). Slot 1 has to disappear from `entries()` without
	/// closing the gap: slot 2's own `slot()` must stay 2, not collapse to 1.
	fn container_with_a_dropped_slot() -> Vec<u8> {
		const HEADER_OFFSET: u16 = 24;
		const SCD_HEADER_LEN: u32 = 28;
		const AUDIO_OFFSET: u32 = HEADER_OFFSET as u32 + SCD_HEADER_LEN;
		const OFFSET_TABLE_LEN: u32 = 3 * 4;
		const SLOT0_DESC: u32 = AUDIO_OFFSET + OFFSET_TABLE_LEN;
		const SLOT2_DESC: u32 = SLOT0_DESC + 32;

		let mut bytes = Vec::new();
		bytes.extend(b"SEDBSSCF"); // magic
		bytes.extend(3u32.to_le_bytes()); // version
		bytes.push(0); // endian
		bytes.push(4); // align
		bytes.extend(HEADER_OFFSET.to_le_bytes());
		bytes.extend(0u64.to_le_bytes()); // file_size, unchecked by the reader

		bytes.extend(0u16.to_le_bytes()); // sound_count
		bytes.extend(0u16.to_le_bytes()); // track_count
		bytes.extend(3u16.to_le_bytes()); // audio_count
		bytes.extend(0u16.to_le_bytes()); // number
		bytes.extend(0u32.to_le_bytes()); // track_offset
		bytes.extend(AUDIO_OFFSET.to_le_bytes());
		bytes.extend(0u32.to_le_bytes()); // layout_offset
		bytes.extend(0u32.to_le_bytes()); // routing_offset
		bytes.extend(0u32.to_le_bytes()); // attribute_offset

		bytes.extend(SLOT0_DESC.to_le_bytes());
		bytes.extend(0u32.to_le_bytes()); // dropped slot
		bytes.extend(SLOT2_DESC.to_le_bytes());

		for _ in 0..2 {
			bytes.extend(0u32.to_le_bytes()); // data_size
			bytes.extend(1u32.to_le_bytes()); // channel_count
			bytes.extend(44100u32.to_le_bytes()); // sample_rate
			bytes.extend((-1i32).to_le_bytes()); // format: Empty
			bytes.extend(0u32.to_le_bytes()); // loop_start
			bytes.extend(0u32.to_le_bytes()); // loop_end
			bytes.extend(0u32.to_le_bytes()); // sub_info_size
			bytes.extend(0u32.to_le_bytes()); // aux_flags
		}
		bytes
	}

	#[test]
	fn a_dropped_zero_offset_does_not_shift_the_slots_after_it() {
		let container = SoundContainer::read(Cursor::new(container_with_a_dropped_slot())).unwrap();
		assert_eq!(container.entries().len(), 2);
		assert_eq!(container.entries()[0].slot(), 0);
		assert_eq!(container.entries()[1].slot(), 2);
	}
}
