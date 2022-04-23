use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for EurekaSphereElementAdjust {
    fn name() -> String {
        "EurekaSphereElementAdjust".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EurekaSphereElementAdjust::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EurekaSphereElementAdjust {
    pub r#power_modifier: u16,
}
impl EurekaSphereElementAdjust {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#power_modifier: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
