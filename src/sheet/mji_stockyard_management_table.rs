use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for MJIStockyardManagementTable {
    fn name() -> String {
        "MJIStockyardManagementTable".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIStockyardManagementTable::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIStockyardManagementTable {
    pub r#material: u8,
}
impl MJIStockyardManagementTable {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#material: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
