use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for PvPSeriesLevel {
    fn name() -> String {
        "PvPSeriesLevel".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PvPSeriesLevel::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PvPSeriesLevel {
    pub r#unknown0: u16,
}
impl PvPSeriesLevel {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
