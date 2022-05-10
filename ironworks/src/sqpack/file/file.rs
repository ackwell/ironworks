use std::io::{Read, Seek, SeekFrom};

use binrw::BinRead;

use crate::error::{Error, Result};

use super::{
	model,
	shared::{read_failed, FileKind, Header},
	standard, texture,
};

pub fn read(mut reader: impl Read + Seek, offset: u32) -> Result<Vec<u8>> {
	// Move to the start of the file and read in the header.
	reader.seek(SeekFrom::Start(offset.into()))?;
	let header = Header::read(&mut reader)?;

	let expected_file_size = header.raw_file_size;

	// TODO: if type 1 and first 64 == second 64, RSF
	//       if type 1 and first 64 == [0..], empty

	let file_offset = offset + header.size;
	let out_buffer = match &header.kind {
		FileKind::Standard => standard::read(reader, file_offset, header),
		FileKind::Model => model::read(reader, file_offset, header),
		FileKind::Texture => texture::read(reader, file_offset, header),
		_ => todo!("File kind: {:?}", header.kind),
	}?;

	match out_buffer.len() == expected_file_size.try_into().unwrap() {
		true => Ok(out_buffer),
		false => Err(Error::Resource(
			read_failed("file", expected_file_size, out_buffer.len()).into(),
		)),
	}
}
