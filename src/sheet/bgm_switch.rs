use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for BGMSwitch {
    fn name() -> String {
        "BGMSwitch".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BGMSwitch::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BGMSwitch {
    pub r#bgm_system_define: u8,
    pub r#quest: u32,
    pub r#unknown2: u8,
    pub r#bgm: u16,
}
impl BGMSwitch {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#bgm_system_define: row.field(0usize + offset)?.into_u8()?,
            r#quest: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#bgm: row.field(3usize + offset)?.into_u16()?,
        })
    }
}
