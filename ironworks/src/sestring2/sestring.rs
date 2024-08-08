use binrw::{binread, helpers::until_exclusive};

// TODO: debug on this should probably be overwritten
#[binread]
#[derive(Debug)]
pub struct SeString {
	#[br(parse_with = until_exclusive(|&byte| byte == 0))]
	data: Vec<u8>,
}

impl SeString {
	pub fn iter(&self) -> PayloadIterator {
		PayloadIterator::new(&self.data)
	}
}

impl<'a> IntoIterator for &'a SeString {
	type Item = Payload<'a>;
	type IntoIter = PayloadIterator<'a>;
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

const MACRO_START: u8 = 0x02;
const MACRO_END: u8 = 0x03;

// TODO: debug on this should probably be overwritten
#[derive(Debug)]
pub struct PayloadIterator<'a> {
	data: &'a [u8],
	offset: usize,
}

impl<'a> PayloadIterator<'a> {
	fn new(data: &'a [u8]) -> Self {
		Self { data, offset: 0 }
	}
}

impl<'a> Iterator for PayloadIterator<'a> {
	type Item = Payload<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		// EOF, stop iteration.
		if self.offset >= self.data.len() {
			return None;
		}

		let rest = &self.data[self.offset..];

		// Next byte is the start of a macro.
		if rest[0] == MACRO_START {
			todo!("MACRO")
		}

		// Otherwise, read plain text until a macro or EOF is detected.
		let text_bytes = match rest.iter().position(|&byte| byte == MACRO_START) {
			Some(position) => &rest[..position],
			None => rest,
		};

		self.offset += text_bytes.len();

		Some(Payload::Text(text_bytes))
	}
}

#[derive(Debug, PartialEq)]
pub enum Payload<'a> {
	Text(&'a [u8]),
	Macro,
}

#[cfg(test)]
mod test {
	use std::fmt;

	use super::*;

	// Yoinked from itertools.
	fn iter_eq<T>(mut a: impl Iterator<Item = T>, mut b: impl Iterator<Item = T>)
	where
		T: fmt::Debug + PartialEq,
	{
		loop {
			match (a.next(), b.next()) {
				(None, None) => return,
				(a_next, b_next) => {
					let equal = match (&a_next, &b_next) {
						(Some(a_val), Some(b_val)) => a_val == b_val,
						_ => false,
					};
					assert!(equal, "{a_next:?} != {b_next:?}")
				}
			}
		}
	}

	#[test]
	fn plain_string() {
		let bytes = b"string";
		let sestring = SeString {
			data: bytes.to_vec(),
		};
		iter_eq(sestring.iter(), [Payload::Text(b"")].into_iter());
	}
}
