use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for IKDRouteTable {
    fn name() -> String {
        "IKDRouteTable".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(IKDRouteTable::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct IKDRouteTable {
    pub r#route: u32,
}
impl IKDRouteTable {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#route: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
