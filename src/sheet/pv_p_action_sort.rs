use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for PvPActionSort {
    fn name() -> String {
        "PvPActionSort".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PvPActionSort::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PvPActionSort {
    pub r#action_type: u8,
    pub r#action: u16,
}
impl PvPActionSort {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#action_type: row.field(0usize + offset)?.into_u8()?,
            r#action: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
