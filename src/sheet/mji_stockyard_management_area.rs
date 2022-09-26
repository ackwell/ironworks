use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for MJIStockyardManagementArea {
    fn name() -> String {
        "MJIStockyardManagementArea".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIStockyardManagementArea::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIStockyardManagementArea {
    pub r#rare_material: u8,
    pub r#unknown1: u8,
    pub r#area: u16,
}
impl MJIStockyardManagementArea {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#rare_material: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#area: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
