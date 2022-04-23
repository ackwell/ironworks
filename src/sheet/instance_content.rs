use ironworks::excel::Row;
use std::vec::Vec;
use std::result::Result;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for InstanceContent {
    fn name() -> String {
        "InstanceContent".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(InstanceContent::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct InstanceContent_BossExp {
    pub r#boss_exp: u32,
}
impl InstanceContent_BossExp {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#boss_exp: row.field(26usize + offset)?.into_u32()?,
        })
    }
}
#[derive(Debug)]
pub struct InstanceContent_BossCurrencyA {
    pub r#boss_currency_a: u16,
}
impl InstanceContent_BossCurrencyA {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#boss_currency_a: row.field(31usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct InstanceContent_BossCurrencyB {
    pub r#boss_currency_b: u16,
}
impl InstanceContent_BossCurrencyB {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#boss_currency_b: row.field(36usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct InstanceContent_BossCurrencyC {
    pub r#boss_currency_c: u16,
}
impl InstanceContent_BossCurrencyC {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#boss_currency_c: row.field(41usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct InstanceContent {
    pub r#instance_content_type: u8,
    pub r#week_restriction: u8,
    pub r#time_limitmin: u16,
    pub r#unknown3: bool,
    pub r#bgm: u16,
    pub r#win_bgm: u16,
    pub r#cutscene: u32,
    pub r#lgb_event_range: u32,
    pub r#order: u16,
    pub r#colosseum: u8,
    pub r#unknown10: bool,
    pub r#instance_content_text_data_boss_start: u32,
    pub r#instance_content_text_data_boss_end: u32,
    pub r#b_npc_base_boss: u32,
    pub r#instance_content_text_data_objective_start: u32,
    pub r#instance_content_text_data_objective_end: u32,
    pub r#sort_key: u16,
    pub r#new_player_bonus_gil: u32,
    pub r#new_player_bonus_exp: u32,
    pub r#new_player_bonus_a: u16,
    pub r#new_player_bonus_b: u16,
    pub r#final_boss_exp: u32,
    pub r#unknown22: u32,
    pub r#final_boss_currency_a: u16,
    pub r#final_boss_currency_b: u16,
    pub r#final_boss_currency_c: u16,
    pub r#boss_exp: Vec<InstanceContent_BossExp>,
    pub r#boss_currency_a: Vec<InstanceContent_BossCurrencyA>,
    pub r#boss_currency_b: Vec<InstanceContent_BossCurrencyB>,
    pub r#boss_currency_c: Vec<InstanceContent_BossCurrencyC>,
    pub r#instance_clear_exp: u32,
    pub r#instance_clear_gil: u32,
    pub r#instance_content_reward_item: u32,
    pub r#unknown49: u8,
    pub r#unknown50: u32,
    pub r#instance_content_buff: i32,
    pub r#unknown52: bool,
    pub r#req_instance: u32,
    pub r#party_condition: i16,
}
impl InstanceContent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#instance_content_type: row.field(0usize + offset)?.into_u8()?,
            r#week_restriction: row.field(1usize + offset)?.into_u8()?,
            r#time_limitmin: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#bgm: row.field(4usize + offset)?.into_u16()?,
            r#win_bgm: row.field(5usize + offset)?.into_u16()?,
            r#cutscene: row.field(6usize + offset)?.into_u32()?,
            r#lgb_event_range: row.field(7usize + offset)?.into_u32()?,
            r#order: row.field(8usize + offset)?.into_u16()?,
            r#colosseum: row.field(9usize + offset)?.into_u8()?,
            r#unknown10: row.field(10usize + offset)?.into_bool()?,
            r#instance_content_text_data_boss_start: row
                .field(11usize + offset)?
                .into_u32()?,
            r#instance_content_text_data_boss_end: row
                .field(12usize + offset)?
                .into_u32()?,
            r#b_npc_base_boss: row.field(13usize + offset)?.into_u32()?,
            r#instance_content_text_data_objective_start: row
                .field(14usize + offset)?
                .into_u32()?,
            r#instance_content_text_data_objective_end: row
                .field(15usize + offset)?
                .into_u32()?,
            r#sort_key: row.field(16usize + offset)?.into_u16()?,
            r#new_player_bonus_gil: row.field(17usize + offset)?.into_u32()?,
            r#new_player_bonus_exp: row.field(18usize + offset)?.into_u32()?,
            r#new_player_bonus_a: row.field(19usize + offset)?.into_u16()?,
            r#new_player_bonus_b: row.field(20usize + offset)?.into_u16()?,
            r#final_boss_exp: row.field(21usize + offset)?.into_u32()?,
            r#unknown22: row.field(22usize + offset)?.into_u32()?,
            r#final_boss_currency_a: row.field(23usize + offset)?.into_u16()?,
            r#final_boss_currency_b: row.field(24usize + offset)?.into_u16()?,
            r#final_boss_currency_c: row.field(25usize + offset)?.into_u16()?,
            r#boss_exp: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(InstanceContent_BossExp::populate(row, offset)?) },
            )?,
            r#boss_currency_a: read_array(
                offset,
                5usize,
                1usize,
                |offset| {
                    Result::Ok(InstanceContent_BossCurrencyA::populate(row, offset)?)
                },
            )?,
            r#boss_currency_b: read_array(
                offset,
                5usize,
                1usize,
                |offset| {
                    Result::Ok(InstanceContent_BossCurrencyB::populate(row, offset)?)
                },
            )?,
            r#boss_currency_c: read_array(
                offset,
                5usize,
                1usize,
                |offset| {
                    Result::Ok(InstanceContent_BossCurrencyC::populate(row, offset)?)
                },
            )?,
            r#instance_clear_exp: row.field(46usize + offset)?.into_u32()?,
            r#instance_clear_gil: row.field(47usize + offset)?.into_u32()?,
            r#instance_content_reward_item: row.field(48usize + offset)?.into_u32()?,
            r#unknown49: row.field(49usize + offset)?.into_u8()?,
            r#unknown50: row.field(50usize + offset)?.into_u32()?,
            r#instance_content_buff: row.field(51usize + offset)?.into_i32()?,
            r#unknown52: row.field(52usize + offset)?.into_bool()?,
            r#req_instance: row.field(53usize + offset)?.into_u32()?,
            r#party_condition: row.field(54usize + offset)?.into_i16()?,
        })
    }
}
