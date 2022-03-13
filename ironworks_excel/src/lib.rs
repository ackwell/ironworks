use std::collections::HashSet;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Invalid resource: {0}")]
	InvalidResource(String),

	#[error("Not found: {0}")]
	NotFound(String),

	#[error(transparent)]
	Downstream(anyhow::Error),
}

// Due to the nature of the ExcelResource trait, it's expected that an anyhow::Error
// returned by a resource function could be a first-party error. To avoid blindly
// bubbling our own errors up as a Downstream, we're manually implementing From
// here and trying to downcast to ourselves - only wrapping in Downstream if that
// is not possible.
impl From<anyhow::Error> for Error {
	fn from(error: anyhow::Error) -> Self {
		match error.downcast::<Error>() {
			Ok(error) => error,
			Err(error) => Error::Downstream(error),
		}
	}
}

pub type ResourceResult<T> = Result<T, anyhow::Error>;

pub trait ExcelResource {
	fn list(&self) -> ResourceResult<ExcelList>;
	// fn header() -> ResourceResult<ExcelHeader>;
	// fn page() -> ResourceResult<ExcelPage>;
}

pub struct Excel<'a> {
	resource: Box<dyn ExcelResource + 'a>,
}

impl<'a> Excel<'a> {
	pub fn new(resource: impl ExcelResource + 'a) -> Self {
		Self {
			resource: Box::new(resource),
		}
	}

	pub fn get_raw_sheet(&self, sheet_name: &str) -> Result<RawExcelSheet, Error> {
		// NOTE: who owns caching this?
		let list = self.resource.list()?;

		if !list.has_sheet(sheet_name) {
			return Err(Error::NotFound(format!("Sheet \"{}\"", sheet_name)));
		}

		Ok(RawExcelSheet {})
	}
}

pub struct ExcelList {
	sheets: HashSet<String>,
}

impl ExcelList {
	// TODO: should this move the bytes?
	pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
		// Binary format is actually just text.
		let mut lines = std::str::from_utf8(bytes)
			.map_err(|error| {
				Error::InvalidResource(format!("Invalid utf8 sequence in ExcelList: {}", error))
			})?
			.split("\r\n");

		// First line is a magic, make sure we've got one.
		match lines.next().map(|line| &line[0..4]) {
			Some("EXLT") => (),
			_ => {
				return Err(Error::InvalidResource(
					"Missing EXLT magic in ExcelList".into(),
				))
			}
		}

		// Build the set of sheets. We're ignoring the sheet ID, as it's pretty
		// irrelevant for us at this stage.
		let sheets = lines
			.filter_map(|line| {
				let index = line.find(',')?;
				Some(line[0..index].to_string())
			})
			.collect::<HashSet<_>>();

		Ok(Self { sheets })
	}

	pub fn has_sheet(&self, sheet_name: &str) -> bool {
		self.sheets.contains(sheet_name)
	}
}

#[derive(Debug)]
pub struct RawExcelSheet {}

struct ExcelHeader {}

struct ExcelPage {}
