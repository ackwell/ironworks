use ironworks::sestring::SeString;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for FCRights {
    fn name() -> String {
        "FCRights".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FCRights::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FCRights {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#icon: u16,
    pub r#fc_rank: u8,
}
impl FCRights {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#icon: row.field(2usize + offset)?.into_u16()?,
            r#fc_rank: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
