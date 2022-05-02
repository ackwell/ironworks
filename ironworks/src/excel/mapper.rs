/// Mapper to fetch file paths for excel lookups.
pub trait Mapper: Send + Sync {
	/// Fetch the path to the excel list file.
	fn exl(&self) -> String;

	/// Fetch the path to a sheet header file.
	fn exh(&self, sheet: &str) -> String;

	/// Fetch the path to a sheet page file.
	fn exd(&self, sheet: &str, start_id: u32, language_id: u8) -> String;
}
