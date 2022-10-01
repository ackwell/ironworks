use crate::{excel::SheetMetadata, file::exh};

use super::{row_options::RowConfig, Sheet};

/// An iterator that iterates over the rows of an excel sheet.
#[derive(Debug)]
pub struct SheetIterator<'i, S> {
	sheet: &'i Sheet<'i, S>,
	config: RowConfig,

	row_id: u32,
	subrow_id: u16,

	subrow_count: Option<u16>,
}

impl<'i, S: SheetMetadata> SheetIterator<'i, S> {
	pub(super) fn new(sheet: &'i Sheet<S>, config: RowConfig) -> Self {
		SheetIterator {
			sheet,
			config,
			row_id: 0,
			subrow_id: 0,
			subrow_count: None,
		}
	}
}

impl<S: SheetMetadata> Iterator for SheetIterator<'_, S> {
	type Item = S::Row;

	fn next(&mut self) -> Option<Self::Item> {
		// TODO: both the .page and .subrow calls should have some means to utilise an iter-wide lang override

		let subrow_count = match self.subrow_count {
			Some(v) => v,
			None => {
				let page = self
					.sheet
					.page(self.row_id, self.subrow_id, self.config.language)
					.ok()?;
				let subrow_count = page.subrow_count(self.row_id).ok()?;
				*self.subrow_count.insert(subrow_count)
			}
		};

		if self.subrow_id >= subrow_count {
			self.row_id += 1;
			self.subrow_id = 0;
			self.subrow_count = None;
		}

		let row = self
			.sheet
			.subrow_with_options(self.row_id, self.subrow_id, self.config.clone())
			.ok()?;

		self.subrow_id += 1;

		Some(row)
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		use exh::SheetKind as K;
		match self.sheet.header().ok()?.kind() {
			// Subrows have to be done the manual way, as there's no way to know the subrow count without reading the relevant .exd page.
			K::Subrows => {
				for _i in 0..n {
					self.next()?;
				}
			}
			_ => self.row_id = n.try_into().unwrap(),
		}

		self.next()
	}
}
