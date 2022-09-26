use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::vec::Vec;
use std::result::Result;
use crate::utility::read_array;
impl MetadataAdapter for TripleTriad {
    fn name() -> String {
        "TripleTriad".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TripleTriad::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TripleTriad {
    pub r#triple_triad_card_fixed: Vec<u16>,
    pub r#triple_triad_card_variable: Vec<u16>,
    pub r#triple_triad_rule: Vec<u8>,
    pub r#uses_regional_rules: bool,
    pub r#fee: u16,
    pub r#previous_quest_join: u8,
    pub r#previous_quest: Vec<u32>,
    pub r#start_time: u16,
    pub r#end_time: u16,
    pub r#default_talk_challenge: u32,
    pub r#default_talk_unavailable: u32,
    pub r#default_talk_npc_win: u32,
    pub r#default_talk_draw: u32,
    pub r#default_talk_pc_win: u32,
    pub r#unknown25: bool,
    pub r#item_possible_reward: Vec<u32>,
}
impl TripleTriad {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#triple_triad_card_fixed: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u16()?) },
            )?,
            r#triple_triad_card_variable: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(5usize + offset)?.into_u16()?) },
            )?,
            r#triple_triad_rule: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(10usize + offset)?.into_u8()?) },
            )?,
            r#uses_regional_rules: row.field(12usize + offset)?.into_bool()?,
            r#fee: row.field(13usize + offset)?.into_u16()?,
            r#previous_quest_join: row.field(14usize + offset)?.into_u8()?,
            r#previous_quest: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(15usize + offset)?.into_u32()?) },
            )?,
            r#start_time: row.field(18usize + offset)?.into_u16()?,
            r#end_time: row.field(19usize + offset)?.into_u16()?,
            r#default_talk_challenge: row.field(20usize + offset)?.into_u32()?,
            r#default_talk_unavailable: row.field(21usize + offset)?.into_u32()?,
            r#default_talk_npc_win: row.field(22usize + offset)?.into_u32()?,
            r#default_talk_draw: row.field(23usize + offset)?.into_u32()?,
            r#default_talk_pc_win: row.field(24usize + offset)?.into_u32()?,
            r#unknown25: row.field(25usize + offset)?.into_bool()?,
            r#item_possible_reward: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(26usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
