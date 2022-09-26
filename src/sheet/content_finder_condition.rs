use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
impl MetadataAdapter for ContentFinderCondition {
    fn name() -> String {
        "ContentFinderCondition".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentFinderCondition::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentFinderCondition {
    pub r#short_code: SeString,
    pub r#territory_type: u16,
    pub r#content_link_type: u8,
    pub r#content: u16,
    pub r#pv_p: bool,
    pub r#unknown5: u8,
    pub r#unknown6: u32,
    pub r#unknown7: u32,
    pub r#accept_class_job_category: u8,
    pub r#content_member_type: u8,
    pub r#unknown10: u8,
    pub r#unknown11: u8,
    pub r#unknown12: u8,
    pub r#unlock_quest: u32,
    pub r#unknown14: u8,
    pub r#unknown15: u32,
    pub r#class_job_level_required: u8,
    pub r#class_job_level_sync: u8,
    pub r#item_level_required: u16,
    pub r#item_level_sync: u16,
    pub r#unknown20: bool,
    pub r#allow_undersized: bool,
    pub r#allow_replacement: bool,
    pub r#unknown23: bool,
    pub r#allow_explorer_mode: bool,
    pub r#unknown25: bool,
    pub r#unknown26: bool,
    pub r#unknown27: u8,
    pub r#unknown28: bool,
    pub r#high_end_duty: bool,
    pub r#unknown30: bool,
    pub r#unknown31: u8,
    pub r#unknown32: bool,
    pub r#unknown33: bool,
    pub r#duty_recorder_allowed: bool,
    pub r#unknown35: bool,
    pub r#unknown36: bool,
    pub r#unknown37: bool,
    pub r#unknown38: bool,
    pub r#name: SeString,
    pub r#name_short: SeString,
    pub r#content_type: u8,
    pub r#transient_key: u8,
    pub r#transient: u32,
    pub r#sort_key: u16,
    pub r#image: u32,
    pub r#icon: u32,
    pub r#unknown47: i8,
    pub r#unknown48: i32,
    pub r#unknown49: bool,
    pub r#level506070_roulette: u8,
    pub r#leveling_roulette: bool,
    pub r#msq_roulette: bool,
    pub r#guild_hest_roulette: bool,
    pub r#expert_roulette: bool,
    pub r#trial_roulette: bool,
    pub r#daily_frontline_challenge: bool,
    pub r#level80_roulette: bool,
    pub r#mentor_roulette: bool,
    pub r#unknown59: bool,
    pub r#unknown60: bool,
    pub r#unknown61: bool,
    pub r#unknown62: bool,
    pub r#unknown63: bool,
    pub r#alliance_roulette: bool,
    pub r#feast_team_roulette: bool,
    pub r#normal_raid_roulette: bool,
}
impl ContentFinderCondition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#short_code: row.field(0usize + offset)?.into_string()?,
            r#territory_type: row.field(1usize + offset)?.into_u16()?,
            r#content_link_type: row.field(2usize + offset)?.into_u8()?,
            r#content: row.field(3usize + offset)?.into_u16()?,
            r#pv_p: row.field(4usize + offset)?.into_bool()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
            r#accept_class_job_category: row.field(8usize + offset)?.into_u8()?,
            r#content_member_type: row.field(9usize + offset)?.into_u8()?,
            r#unknown10: row.field(10usize + offset)?.into_u8()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
            r#unlock_quest: row.field(13usize + offset)?.into_u32()?,
            r#unknown14: row.field(14usize + offset)?.into_u8()?,
            r#unknown15: row.field(15usize + offset)?.into_u32()?,
            r#class_job_level_required: row.field(16usize + offset)?.into_u8()?,
            r#class_job_level_sync: row.field(17usize + offset)?.into_u8()?,
            r#item_level_required: row.field(18usize + offset)?.into_u16()?,
            r#item_level_sync: row.field(19usize + offset)?.into_u16()?,
            r#unknown20: row.field(20usize + offset)?.into_bool()?,
            r#allow_undersized: row.field(21usize + offset)?.into_bool()?,
            r#allow_replacement: row.field(22usize + offset)?.into_bool()?,
            r#unknown23: row.field(23usize + offset)?.into_bool()?,
            r#allow_explorer_mode: row.field(24usize + offset)?.into_bool()?,
            r#unknown25: row.field(25usize + offset)?.into_bool()?,
            r#unknown26: row.field(26usize + offset)?.into_bool()?,
            r#unknown27: row.field(27usize + offset)?.into_u8()?,
            r#unknown28: row.field(28usize + offset)?.into_bool()?,
            r#high_end_duty: row.field(29usize + offset)?.into_bool()?,
            r#unknown30: row.field(30usize + offset)?.into_bool()?,
            r#unknown31: row.field(31usize + offset)?.into_u8()?,
            r#unknown32: row.field(32usize + offset)?.into_bool()?,
            r#unknown33: row.field(33usize + offset)?.into_bool()?,
            r#duty_recorder_allowed: row.field(34usize + offset)?.into_bool()?,
            r#unknown35: row.field(35usize + offset)?.into_bool()?,
            r#unknown36: row.field(36usize + offset)?.into_bool()?,
            r#unknown37: row.field(37usize + offset)?.into_bool()?,
            r#unknown38: row.field(38usize + offset)?.into_bool()?,
            r#name: row.field(39usize + offset)?.into_string()?,
            r#name_short: row.field(40usize + offset)?.into_string()?,
            r#content_type: row.field(41usize + offset)?.into_u8()?,
            r#transient_key: row.field(42usize + offset)?.into_u8()?,
            r#transient: row.field(43usize + offset)?.into_u32()?,
            r#sort_key: row.field(44usize + offset)?.into_u16()?,
            r#image: row.field(45usize + offset)?.into_u32()?,
            r#icon: row.field(46usize + offset)?.into_u32()?,
            r#unknown47: row.field(47usize + offset)?.into_i8()?,
            r#unknown48: row.field(48usize + offset)?.into_i32()?,
            r#unknown49: row.field(49usize + offset)?.into_bool()?,
            r#level506070_roulette: row.field(50usize + offset)?.into_u8()?,
            r#leveling_roulette: row.field(51usize + offset)?.into_bool()?,
            r#msq_roulette: row.field(52usize + offset)?.into_bool()?,
            r#guild_hest_roulette: row.field(53usize + offset)?.into_bool()?,
            r#expert_roulette: row.field(54usize + offset)?.into_bool()?,
            r#trial_roulette: row.field(55usize + offset)?.into_bool()?,
            r#daily_frontline_challenge: row.field(56usize + offset)?.into_bool()?,
            r#level80_roulette: row.field(57usize + offset)?.into_bool()?,
            r#mentor_roulette: row.field(58usize + offset)?.into_bool()?,
            r#unknown59: row.field(59usize + offset)?.into_bool()?,
            r#unknown60: row.field(60usize + offset)?.into_bool()?,
            r#unknown61: row.field(61usize + offset)?.into_bool()?,
            r#unknown62: row.field(62usize + offset)?.into_bool()?,
            r#unknown63: row.field(63usize + offset)?.into_bool()?,
            r#alliance_roulette: row.field(64usize + offset)?.into_bool()?,
            r#feast_team_roulette: row.field(65usize + offset)?.into_bool()?,
            r#normal_raid_roulette: row.field(66usize + offset)?.into_bool()?,
        })
    }
}
