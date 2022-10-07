use crate::{excel::SheetMetadata, file::exh};

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
		// Get the metadata for this iteration.
		let header = self.sheet.header().ok()?;
		let pages = header.pages();

		// If we're past the end of the available pages, stop the iterator.
		let page_definition = pages.get(self.page_index)?;
		let row_id = page_definition.start_id() + self.row_offset;

		// Fetch the row for this iteration's result.
		let row = self
			.sheet
			.subrow_with_options(row_id, self.subrow_id, self.config.clone())
			.ok()?;

		// Fetch the count of subrows for this row. It's cached to avoid subrow sheets requiring multiple lookups.
		let subrow_count = match self.sheet.kind().ok()? {
			exh::SheetKind::Subrows => match self.subrow_count {
				Some(value) => value,
				None => {
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

		self.subrow_id += 1;

		// If the subrow bounds have been exceeded, move on to the next row.
		if self.subrow_id >= subrow_count {
			self.subrow_id = 0;
			self.subrow_count = None;
			self.row_offset += 1;
		}

		// If the page bounds have been exceeded, move on to the next page.
		if self.row_offset >= page_definition.row_count() {
			self.row_offset = 0;
			self.page_index += 1;
		}

		Some(row)
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		let header = self.sheet.header().ok()?;

		match header.kind() {
			// Subrows have to be done the manual way, as there's no way to know the subrow count without reading the relevant .exd page.
			exh::SheetKind::Subrows => {
				for _i in 0..n {
					self.next()?;
				}
			}
			_ => {
				let pages = header.pages();

				// Get the expected offset from the current row position.
				let mut new_offset = self.row_offset + u32::try_from(n).unwrap();

				// Starting with the current page, skip over pages until the new offset is within a page bounds.
				for definition in pages.iter().skip(self.page_index) {
					let row_count = definition.row_count();

					if new_offset < row_count {
						break;
					}

					self.page_index += 1;
					new_offset -= row_count;
				}

				// If there are no more pages remaining, end the iterator.
				if self.page_index >= pages.len() {
					return None;
				}

				// Skip was successful, update the row offset within the new page index.
				self.row_offset = new_offset;
			}
		}

		self.next()
	}
}
