use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for JobHudManual {
    fn name() -> String {
        "JobHudManual".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(JobHudManual::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct JobHudManual {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#action: u32,
    pub r#unknown3: u8,
    pub r#unknown4: u32,
    pub r#guide: u16,
}
impl JobHudManual {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#action: row.field(2usize + offset)?.into_u32()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
            r#guide: row.field(5usize + offset)?.into_u16()?,
        })
    }
}
