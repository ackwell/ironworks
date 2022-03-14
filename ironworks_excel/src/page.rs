use crate::error::Result;

#[derive(Debug)]
pub struct ExcelPage {}

impl ExcelPage {
	pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
		Ok(Self {})
	}
}
