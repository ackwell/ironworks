use std::sync::Arc;

use crate::{
	error::{Error, ErrorValue, Result},
	file::{exd, exh},
};

use super::{metadata::SheetMetadata, sheet::Sheet};

/// Iterator over the rows in a sheet.
#[derive(Debug)]
pub struct SheetIterator<S> {
	sheet: Sheet<S>,

	page_index: usize,
	row_index: usize,
	subrow_id: u16,

	subrow_max: Option<u16>,
}

impl<S: SheetMetadata> SheetIterator<S> {
	pub(super) fn new(sheet: Sheet<S>) -> Self {
		Self {
			sheet,

			page_index: 0,
			row_index: 0,
			subrow_id: 0,

			subrow_max: None,
		}
	}
}

impl<S: SheetMetadata> Iterator for SheetIterator<S> {
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
			row = self.sheet.subrow(self.row_id().ok()?, self.subrow_id);
			self.step().ok()?;
		}

		row.ok()
	}
}

impl<S: SheetMetadata> SheetIterator<S> {
	fn step(&mut self) -> Result<()> {
		self.subrow_id += 1;

		// If the subrow bounds have been exceeded, move on to the next row.
		if self.subrow_id > self.subrow_max()? {
			self.subrow_id = 0;
			self.subrow_max = None;
			self.row_index += 1;
		}

		// If the page bounds have been exceeded, move on to the next page.
		if self.row_index >= self.page()?.rows().len() {
			self.row_index = 0;
			self.page_index += 1;
		}

		Ok(())
	}

	fn subrow_max(&mut self) -> Result<u16> {
		// Fetch the count of subrows for this row. It's cached to avoid subrow sheets requiring multiple lookups.
		let count = match self.sheet.kind()? {
			exh::SheetKind::Subrows => match self.subrow_max {
				Some(value) => value,
				None => {
					// TODO: this is reading the page out twice, which is really dumb. Expose more data via exd and move logic to excel to avoid this shit.
					let row_id = self.row_id()?;
					let page = self.page()?;

					// If we get a row not found, we can assume that there are "zero" subrows, in an effort to skip this row.
					let subrow_max = match page.subrow_max(row_id) {
						Err(Error::NotFound(ErrorValue::Row { .. })) => Ok(0),
						other => other,
					}
					.expect("failed to read subrow count while iterating");

					*self.subrow_max.insert(subrow_max)
				}
			},
			_ => 1,
		};
		Ok(count)
	}

	fn row_id(&self) -> Result<u32> {
		let id = self
			.page()?
			.rows()
			.get(self.row_index)
			.ok_or_else(|| Error::NotFound(ErrorValue::Other(format!("Row {}", self.row_index))))?
			.id();

		Ok(id)
	}

	fn page(&self) -> Result<Arc<exd::ExcelData>> {
		self.sheet.page(
			self.page_definition()?.start_id(),
			self.sheet.resolve_language(self.sheet.default_language)?,
		)
	}

	fn page_definition(&self) -> Result<exh::PageDefinition> {
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
