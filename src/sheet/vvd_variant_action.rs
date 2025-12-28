use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for VVDVariantAction {
    fn name() -> String {
        "VVDVariantAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(VVDVariantAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct VVDVariantAction {
    pub r#action: u32,
}
impl VVDVariantAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#action: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
