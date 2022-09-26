use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for MJICraftworksSupplyDefine {
    fn name() -> String {
        "MJICraftworksSupplyDefine".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJICraftworksSupplyDefine::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJICraftworksSupplyDefine {
    pub r#supply: i16,
    pub r#ratio: u16,
}
impl MJICraftworksSupplyDefine {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#supply: row.field(0usize + offset)?.into_i16()?,
            r#ratio: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
