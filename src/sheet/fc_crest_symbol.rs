use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for FCCrestSymbol {
    fn name() -> String {
        "FCCrestSymbol".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FCCrestSymbol::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FCCrestSymbol {
    pub r#color_num: u8,
    pub r#fc_right: u8,
}
impl FCCrestSymbol {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#color_num: row.field(0usize + offset)?.into_u8()?,
            r#fc_right: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
