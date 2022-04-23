use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for NotoriousMonster {
    fn name() -> String {
        "NotoriousMonster".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(NotoriousMonster::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct NotoriousMonster {
    pub r#b_npc_base: i32,
    pub r#rank: u8,
    pub r#b_npc_name: u32,
}
impl NotoriousMonster {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#b_npc_base: row.field(0usize + offset)?.into_i32()?,
            r#rank: row.field(1usize + offset)?.into_u8()?,
            r#b_npc_name: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
