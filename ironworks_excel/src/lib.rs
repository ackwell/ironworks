use std::collections::HashSet;

pub type ResourceResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait ExcelResource {
	fn list(&self) -> ResourceResult<ExcelList>;
	// fn header() -> ResourceResult<ExcelHeader>;
	// fn page() -> ResourceResult<ExcelPage>;
}

struct Excel {}

pub struct ExcelList {
	sheets: HashSet<String>,
}

impl ExcelList {
	pub fn from_bytes(bytes: &[u8]) -> Self {
		// TODO: Error handling

		// Binary format is actually just text.
		let mut lines = std::str::from_utf8(bytes).unwrap().split("\r\n");

		// First line is a magic, make sure we've got one.
		match lines.next().map(|line| &line[0..4]) {
			Some("EXLT") => (),
			_ => panic!("magic missing"),
		}

		// Build the set of sheets. We're ignoring the sheet ID, as it's pretty
		// irrelevant for us at this stage.
		let sheets = lines
			.filter_map(|line| {
				let index = line.find(',')?;
				Some(line[0..index].to_string())
			})
			.collect::<HashSet<_>>();

		Self { sheets }
	}

	pub fn has_sheet(&self, sheet_name: &str) -> bool {
		self.sheets.contains(sheet_name)
	}
}

struct ExcelSheet {}

struct ExcelHeader {}

struct ExcelPage {}
