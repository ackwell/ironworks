use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
impl MetadataAdapter for GcArmyExpeditionType {
    fn name() -> String {
        "GcArmyExpeditionType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GcArmyExpeditionType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GcArmyExpeditionType {
    pub r#name: SeString,
}
impl GcArmyExpeditionType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
