use std::rc::Rc;

use binrw::BinRead;

use crate::{error::Result, sqpack::Resource};

use super::index1::Index1;

// do i just trait this?
// or would making it a trait make the reader, and then the sqpack, need to generic over it
// in that case wrapper makes more sense i guess

// tempted to say index owns chunks and then it can return file locations like the old one but with less wiring

// with the binary reading stuff this should probably be split up into a few files

#[derive(Debug)]
pub struct Index<R> {
	resource: Rc<R>,
}

impl<R: Resource> Index<R> {
	pub fn new(resource: Rc<R>) -> Result<Self> {
		// TODO: handle chunks

		// ergh does this mean we need to pass the meta down here too? this is getting messy.
		let mut foo = resource.index(0, 10, 0)?;
		let fsdf = Index1::read(&mut foo);
		println!("uuuuuh... something? {fsdf:#?}");

		Ok(Self { resource })
	}
}
