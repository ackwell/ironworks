use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Buddy {
    fn name() -> String {
        "Buddy".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Buddy::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Buddy {
    pub r#base: u8,
    pub r#quest_requirement2: i32,
    pub r#quest_requirement1: i32,
    pub r#base_equip: i32,
    pub r#sound_effect4: SeString,
    pub r#sound_effect3: SeString,
    pub r#sound_effect2: SeString,
    pub r#sound_effect1: SeString,
}
impl Buddy {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#base: row.field(0usize + offset)?.into_u8()?,
            r#quest_requirement2: row.field(1usize + offset)?.into_i32()?,
            r#quest_requirement1: row.field(2usize + offset)?.into_i32()?,
            r#base_equip: row.field(3usize + offset)?.into_i32()?,
            r#sound_effect4: row.field(4usize + offset)?.into_string()?,
            r#sound_effect3: row.field(5usize + offset)?.into_string()?,
            r#sound_effect2: row.field(6usize + offset)?.into_string()?,
            r#sound_effect1: row.field(7usize + offset)?.into_string()?,
        })
    }
}
