use std::vec::Vec;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for HWDCrafterSupply {
    fn name() -> String {
        "HWDCrafterSupply".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDCrafterSupply::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDCrafterSupply {
    pub r#item_trade_in: Vec<u32>,
    pub r#level: Vec<u8>,
    pub r#level_max: Vec<u8>,
    pub r#unknown69: u8,
    pub r#unknown70: u8,
    pub r#unknown71: u8,
    pub r#unknown72: u8,
    pub r#unknown73: u8,
    pub r#unknown74: u8,
    pub r#unknown75: u8,
    pub r#unknown76: u8,
    pub r#unknown77: u8,
    pub r#unknown78: u8,
    pub r#unknown79: u8,
    pub r#unknown80: u8,
    pub r#unknown81: u8,
    pub r#unknown82: u8,
    pub r#unknown83: u8,
    pub r#unknown84: u8,
    pub r#unknown85: u8,
    pub r#unknown86: u8,
    pub r#unknown87: u8,
    pub r#unknown88: u8,
    pub r#unknown89: u8,
    pub r#unknown90: u8,
    pub r#unknown91: u8,
    pub r#base_collectable_rating: Vec<u16>,
    pub r#mid_collectable_rating: Vec<u16>,
    pub r#high_collectable_rating: Vec<u16>,
    pub r#base_collectable_reward: Vec<u16>,
    pub r#mid_collectable_reward: Vec<u16>,
    pub r#high_collectable_reward: Vec<u16>,
    pub r#base_collectable_reward_post_phase: Vec<u16>,
    pub r#mid_collectable_reward_post_phase: Vec<u16>,
    pub r#high_collectable_reward_post_phase: Vec<u16>,
    pub r#term_name: Vec<u8>,
}
impl HWDCrafterSupply {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item_trade_in: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
            r#level: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(23usize + offset)?.into_u8()?) },
            )?,
            r#level_max: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(46usize + offset)?.into_u8()?) },
            )?,
            r#unknown69: row.field(69usize + offset)?.into_u8()?,
            r#unknown70: row.field(70usize + offset)?.into_u8()?,
            r#unknown71: row.field(71usize + offset)?.into_u8()?,
            r#unknown72: row.field(72usize + offset)?.into_u8()?,
            r#unknown73: row.field(73usize + offset)?.into_u8()?,
            r#unknown74: row.field(74usize + offset)?.into_u8()?,
            r#unknown75: row.field(75usize + offset)?.into_u8()?,
            r#unknown76: row.field(76usize + offset)?.into_u8()?,
            r#unknown77: row.field(77usize + offset)?.into_u8()?,
            r#unknown78: row.field(78usize + offset)?.into_u8()?,
            r#unknown79: row.field(79usize + offset)?.into_u8()?,
            r#unknown80: row.field(80usize + offset)?.into_u8()?,
            r#unknown81: row.field(81usize + offset)?.into_u8()?,
            r#unknown82: row.field(82usize + offset)?.into_u8()?,
            r#unknown83: row.field(83usize + offset)?.into_u8()?,
            r#unknown84: row.field(84usize + offset)?.into_u8()?,
            r#unknown85: row.field(85usize + offset)?.into_u8()?,
            r#unknown86: row.field(86usize + offset)?.into_u8()?,
            r#unknown87: row.field(87usize + offset)?.into_u8()?,
            r#unknown88: row.field(88usize + offset)?.into_u8()?,
            r#unknown89: row.field(89usize + offset)?.into_u8()?,
            r#unknown90: row.field(90usize + offset)?.into_u8()?,
            r#unknown91: row.field(91usize + offset)?.into_u8()?,
            r#base_collectable_rating: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(92usize + offset)?.into_u16()?) },
            )?,
            r#mid_collectable_rating: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(115usize + offset)?.into_u16()?) },
            )?,
            r#high_collectable_rating: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(138usize + offset)?.into_u16()?) },
            )?,
            r#base_collectable_reward: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(161usize + offset)?.into_u16()?) },
            )?,
            r#mid_collectable_reward: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(184usize + offset)?.into_u16()?) },
            )?,
            r#high_collectable_reward: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(207usize + offset)?.into_u16()?) },
            )?,
            r#base_collectable_reward_post_phase: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(230usize + offset)?.into_u16()?) },
            )?,
            r#mid_collectable_reward_post_phase: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(253usize + offset)?.into_u16()?) },
            )?,
            r#high_collectable_reward_post_phase: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(276usize + offset)?.into_u16()?) },
            )?,
            r#term_name: read_array(
                offset,
                23usize,
                1usize,
                |offset| { Result::Ok(row.field(299usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
