use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ENpcDressUp {
    fn name() -> String {
        "ENpcDressUp".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ENpcDressUp::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ENpcDressUp {
    pub r#unknown0: u8,
    pub r#e_npc_dress_up_dress: u8,
}
impl ENpcDressUp {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#e_npc_dress_up_dress: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
