use ironworks::sestring::SeString;
use crate::error::PopulateError;
use std::result::Result;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::utility::read_array;
impl MetadataAdapter for QuestBattle {
    fn name() -> String {
        "QuestBattle".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestBattle::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestBattle {
    pub r#quest: i32,
    pub r#quest_battle_scene: u8,
    pub r#time_limit: u16,
    pub r#level_sync: u16,
    pub r#script_instruction: Vec<SeString>,
    pub r#script_value: Vec<u32>,
}
impl QuestBattle {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_i32()?,
            r#quest_battle_scene: row.field(1usize + offset)?.into_u8()?,
            r#time_limit: row.field(2usize + offset)?.into_u16()?,
            r#level_sync: row.field(3usize + offset)?.into_u16()?,
            r#script_instruction: read_array(
                offset,
                200usize,
                1usize,
                |offset| { Result::Ok(row.field(4usize + offset)?.into_string()?) },
            )?,
            r#script_value: read_array(
                offset,
                200usize,
                1usize,
                |offset| { Result::Ok(row.field(204usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
