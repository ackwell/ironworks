use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for DynamicEventSingleBattle {
    fn name() -> String {
        "DynamicEventSingleBattle".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DynamicEventSingleBattle::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DynamicEventSingleBattle {
    pub r#b_npc_name: i32,
    pub r#icon: u32,
    pub r#text: SeString,
}
impl DynamicEventSingleBattle {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#b_npc_name: row.field(0usize + offset)?.into_i32()?,
            r#icon: row.field(1usize + offset)?.into_u32()?,
            r#text: row.field(2usize + offset)?.into_string()?,
        })
    }
}
