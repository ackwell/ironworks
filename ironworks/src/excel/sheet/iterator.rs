use std::fmt::Debug;

use crate::{
	error::{Error, ErrorValue, Result},
	excel::SheetMetadata,
	file::exh::{self, PageDefinition},
};

use super::{row_options::RowConfig, Sheet};

/// An iterator that iterates over the rows of an excel sheet.
#[derive(Debug)]
pub struct SheetIterator<'i, S> {
	sheet: &'i Sheet<'i, S>,
	config: RowConfig,

	page_index: usize,
	row_offset: u32,
	subrow_id: u16,

	subrow_count: Option<u16>,
}

impl<'i, S: SheetMetadata> SheetIterator<'i, S> {
	pub(super) fn new(sheet: &'i Sheet<S>, config: RowConfig) -> Self {
		SheetIterator {
			sheet,
			config,

			page_index: 0,
			row_offset: 0,
			subrow_id: 0,

			subrow_count: None,
		}
	}
}

impl<S: SheetMetadata> Iterator for SheetIterator<'_, S> {
	type Item = S::Row;

	fn next(&mut self) -> Option<Self::Item> {
		// If we've walked past the last page, stop the iterator.
		let page_count = self.sheet.header().ok()?.pages().len();
		if self.page_index >= page_count {
			return None;
		}

		let mut row = Err(Error::NotFound(ErrorValue::Row {
			row: 0,
			subrow: 0,
			sheet: None,
		}));

		while let Err(Error::NotFound(ErrorValue::Row { .. })) = row {
			row = self.sheet.subrow_with_options(
				self.row_id().ok()?,
				self.subrow_id,
				self.config.clone(),
			);
			self.step().ok()?;
		}

		row.ok()
	}
}

impl<S: SheetMetadata> SheetIterator<'_, S> {
	fn step(&mut self) -> Result<()> {
		self.subrow_id += 1;

		// If the subrow bounds have been exceeded, move on to the next row.
		if self.subrow_id >= self.subrow_count()? {
			self.subrow_id = 0;
			self.subrow_count = None;
			self.row_offset += 1;
		}

		// If the page bounds have been exceeded, move on to the next page.
		if self.row_offset >= self.page_definition()?.row_count() {
			self.row_offset = 0;
			self.page_index += 1;
		}

		Ok(())
	}

	fn subrow_count(&mut self) -> Result<u16> {
		// Fetch the count of subrows for this row. It's cached to avoid subrow sheets requiring multiple lookups.
		let count = match self.sheet.kind()? {
			exh::SheetKind::Subrows => match self.subrow_count {
				Some(value) => value,
				None => {
					let row_id = self.row_id()?;
					let page = self
						.sheet
						.page(row_id, self.subrow_id, self.config.language)
						.expect("failed to read page while iterating");
					let subrow_count = page
						.subrow_count(row_id)
						.expect("failed to read subrow count while iterating");
					*self.subrow_count.insert(subrow_count)
				}
			},
			_ => 1,
		};
		Ok(count)
	}

	fn row_id(&self) -> Result<u32> {
		Ok(self.page_definition()?.start_id() + self.row_offset)
	}

	fn page_definition(&self) -> Result<PageDefinition> {
		// Get the metadata for this iteration.
		let header = self.sheet.header()?;
		let pages = header.pages();

		// If we're past the end of the available pages, stop the iterator.
		pages
			.get(self.page_index)
			.ok_or_else(|| Error::NotFound(ErrorValue::Other(format!("Page {}", self.page_index))))
			.copied()
	}
}
