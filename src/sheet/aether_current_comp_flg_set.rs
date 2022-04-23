use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::utility::read_array;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for AetherCurrentCompFlgSet {
    fn name() -> String {
        "AetherCurrentCompFlgSet".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AetherCurrentCompFlgSet::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AetherCurrentCompFlgSet {
    pub r#territory: i32,
    pub r#aether_current: Vec<i32>,
}
impl AetherCurrentCompFlgSet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#territory: row.field(0usize + offset)?.into_i32()?,
            r#aether_current: read_array(
                offset,
                15usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_i32()?) },
            )?,
        })
    }
}
