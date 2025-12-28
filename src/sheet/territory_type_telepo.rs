use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for TerritoryTypeTelepo {
    fn name() -> String {
        "TerritoryTypeTelepo".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TerritoryTypeTelepo::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TerritoryTypeTelepo {
    pub r#x: u16,
    pub r#y: u16,
    pub r#expansion: u16,
    pub r#telepo_relay: u8,
}
impl TerritoryTypeTelepo {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#x: row.field(0usize + offset)?.into_u16()?,
            r#y: row.field(1usize + offset)?.into_u16()?,
            r#expansion: row.field(2usize + offset)?.into_u16()?,
            r#telepo_relay: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
