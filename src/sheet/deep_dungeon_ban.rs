use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for DeepDungeonBan {
    fn name() -> String {
        "DeepDungeonBan".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DeepDungeonBan::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DeepDungeonBan {
    pub r#screen_image: u16,
    pub r#log_message: u16,
    pub r#name: u16,
}
impl DeepDungeonBan {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#screen_image: row.field(0usize + offset)?.into_u16()?,
            r#log_message: row.field(1usize + offset)?.into_u16()?,
            r#name: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
