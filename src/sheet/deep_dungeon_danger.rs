use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for DeepDungeonDanger {
    fn name() -> String {
        "DeepDungeonDanger".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DeepDungeonDanger::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DeepDungeonDanger {
    pub r#screen_image: u16,
    pub r#log_message: u16,
    pub r#name: u16,
}
impl DeepDungeonDanger {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#screen_image: row.field(0usize + offset)?.into_u16()?,
            r#log_message: row.field(1usize + offset)?.into_u16()?,
            r#name: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
