use std::{
	io::SeekFrom,
	ops::Deref,
	sync::{Arc, Mutex},
};

use binrw::{meta::ReadEndian, BinRead};
use derivative::Derivative;
use lazy_init::Lazy;

use crate::FileStream;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct LazyStreamReader<T> {
	offset: u64,
	value: Lazy<T>,
	#[derivative(Debug = "ignore")]
	stream: Arc<Mutex<Box<dyn FileStream>>>,
}

impl<T> LazyStreamReader<T> {
	pub(super) fn new(stream: Arc<Mutex<Box<dyn FileStream>>>) -> Self {
		let mut handle = stream.lock().expect("e");
		let offset = handle.stream_position().expect("e");
		drop(handle);

		Self {
			offset,
			value: Default::default(),
			stream,
		}
	}
}

impl<T: BinRead<Args = ()> + ReadEndian> Deref for LazyStreamReader<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.value.get_or_create(|| {
			let mut handle = self.stream.lock().expect("e");
			handle.seek(SeekFrom::Start(self.offset)).expect("e");
			T::read(&mut *handle).expect("e")
		})
	}
}
