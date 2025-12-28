use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for TreasureModel {
    fn name() -> String {
        "TreasureModel".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TreasureModel::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TreasureModel {
    pub r#path: SeString,
}
impl TreasureModel {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#path: row.field(0usize + offset)?.into_string()?,
        })
    }
}
