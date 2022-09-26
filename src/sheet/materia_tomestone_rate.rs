use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MateriaTomestoneRate {
    fn name() -> String {
        "MateriaTomestoneRate".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MateriaTomestoneRate::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MateriaTomestoneRate {
    pub r#rate: u32,
}
impl MateriaTomestoneRate {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#rate: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
