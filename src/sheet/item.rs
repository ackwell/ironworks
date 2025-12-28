use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for Item {
    fn name() -> String {
        "Item".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Item::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Item_BaseParam {
    pub r#base_param: u8,
    pub r#base_param_value: i16,
}
impl Item_BaseParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#base_param: row.field(59usize + offset)?.into_u8()?,
            r#base_param_value: row.field(60usize + offset)?.into_i16()?,
        })
    }
}
#[derive(Debug)]
pub struct Item_BaseParamSpecial {
    pub r#base_param_special: u8,
    pub r#base_param_value_special: i16,
}
impl Item_BaseParamSpecial {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#base_param_special: row.field(73usize + offset)?.into_u8()?,
            r#base_param_value_special: row.field(74usize + offset)?.into_i16()?,
        })
    }
}
#[derive(Debug)]
pub struct Item {
    pub r#singular: SeString,
    pub r#adjective: i8,
    pub r#plural: SeString,
    pub r#possessive_pronoun: i8,
    pub r#starts_with_vowel: i8,
    pub r#unknown5: i8,
    pub r#pronoun: i8,
    pub r#article: i8,
    pub r#description: SeString,
    pub r#name: SeString,
    pub r#icon: u16,
    pub r#level_item: u16,
    pub r#rarity: u8,
    pub r#filter_group: u8,
    pub r#additional_data: u32,
    pub r#item_ui_category: u8,
    pub r#item_search_category: u8,
    pub r#equip_slot_category: u8,
    pub r#item_sort_category: u8,
    pub r#unknown19: u16,
    pub r#stack_size: u32,
    pub r#is_unique: bool,
    pub r#is_untradable: bool,
    pub r#is_indisposable: bool,
    pub r#lot: bool,
    pub r#price_mid: u32,
    pub r#price_low: u32,
    pub r#can_be_hq: bool,
    pub r#dye_count: u8,
    pub r#is_crest_worthy: bool,
    pub r#item_action: u16,
    pub r#cast_times: u8,
    pub r#cooldowns: u16,
    pub r#class_job_repair: u8,
    pub r#item_repair: i32,
    pub r#item_glamour: i32,
    pub r#desynth: u16,
    pub r#is_collectable: bool,
    pub r#always_collectable: bool,
    pub r#aetherial_reduce: u16,
    pub r#level_equip: u8,
    pub r#required_pvp_rank: u8,
    pub r#equip_restriction: u8,
    pub r#class_job_category: u8,
    pub r#grand_company: u8,
    pub r#item_series: u8,
    pub r#base_param_modifier: u8,
    pub r#model_main: u64,
    pub r#model_sub: u64,
    pub r#class_job_use: u8,
    pub r#unknown50: u8,
    pub r#damage_phys: u16,
    pub r#damage_mag: u16,
    pub r#delayms: u16,
    pub r#unknown54: u8,
    pub r#block_rate: u16,
    pub r#block: u16,
    pub r#defense_phys: u16,
    pub r#defense_mag: u16,
    pub r#base_param: Vec<Item_BaseParam>,
    pub r#item_special_bonus: u8,
    pub r#item_special_bonus_param: u8,
    pub r#base_param_special: Vec<Item_BaseParamSpecial>,
    pub r#materialize_type: u8,
    pub r#materia_slot_count: u8,
    pub r#is_advanced_melding_permitted: bool,
    pub r#is_pv_p: bool,
    pub r#sub_stat_category: u8,
    pub r#is_glamourous: bool,
}
impl Item {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#singular: row.field(0usize + offset)?.into_string()?,
            r#adjective: row.field(1usize + offset)?.into_i8()?,
            r#plural: row.field(2usize + offset)?.into_string()?,
            r#possessive_pronoun: row.field(3usize + offset)?.into_i8()?,
            r#starts_with_vowel: row.field(4usize + offset)?.into_i8()?,
            r#unknown5: row.field(5usize + offset)?.into_i8()?,
            r#pronoun: row.field(6usize + offset)?.into_i8()?,
            r#article: row.field(7usize + offset)?.into_i8()?,
            r#description: row.field(8usize + offset)?.into_string()?,
            r#name: row.field(9usize + offset)?.into_string()?,
            r#icon: row.field(10usize + offset)?.into_u16()?,
            r#level_item: row.field(11usize + offset)?.into_u16()?,
            r#rarity: row.field(12usize + offset)?.into_u8()?,
            r#filter_group: row.field(13usize + offset)?.into_u8()?,
            r#additional_data: row.field(14usize + offset)?.into_u32()?,
            r#item_ui_category: row.field(15usize + offset)?.into_u8()?,
            r#item_search_category: row.field(16usize + offset)?.into_u8()?,
            r#equip_slot_category: row.field(17usize + offset)?.into_u8()?,
            r#item_sort_category: row.field(18usize + offset)?.into_u8()?,
            r#unknown19: row.field(19usize + offset)?.into_u16()?,
            r#stack_size: row.field(20usize + offset)?.into_u32()?,
            r#is_unique: row.field(21usize + offset)?.into_bool()?,
            r#is_untradable: row.field(22usize + offset)?.into_bool()?,
            r#is_indisposable: row.field(23usize + offset)?.into_bool()?,
            r#lot: row.field(24usize + offset)?.into_bool()?,
            r#price_mid: row.field(25usize + offset)?.into_u32()?,
            r#price_low: row.field(26usize + offset)?.into_u32()?,
            r#can_be_hq: row.field(27usize + offset)?.into_bool()?,
            r#dye_count: row.field(28usize + offset)?.into_u8()?,
            r#is_crest_worthy: row.field(29usize + offset)?.into_bool()?,
            r#item_action: row.field(30usize + offset)?.into_u16()?,
            r#cast_times: row.field(31usize + offset)?.into_u8()?,
            r#cooldowns: row.field(32usize + offset)?.into_u16()?,
            r#class_job_repair: row.field(33usize + offset)?.into_u8()?,
            r#item_repair: row.field(34usize + offset)?.into_i32()?,
            r#item_glamour: row.field(35usize + offset)?.into_i32()?,
            r#desynth: row.field(36usize + offset)?.into_u16()?,
            r#is_collectable: row.field(37usize + offset)?.into_bool()?,
            r#always_collectable: row.field(38usize + offset)?.into_bool()?,
            r#aetherial_reduce: row.field(39usize + offset)?.into_u16()?,
            r#level_equip: row.field(40usize + offset)?.into_u8()?,
            r#required_pvp_rank: row.field(41usize + offset)?.into_u8()?,
            r#equip_restriction: row.field(42usize + offset)?.into_u8()?,
            r#class_job_category: row.field(43usize + offset)?.into_u8()?,
            r#grand_company: row.field(44usize + offset)?.into_u8()?,
            r#item_series: row.field(45usize + offset)?.into_u8()?,
            r#base_param_modifier: row.field(46usize + offset)?.into_u8()?,
            r#model_main: row.field(47usize + offset)?.into_u64()?,
            r#model_sub: row.field(48usize + offset)?.into_u64()?,
            r#class_job_use: row.field(49usize + offset)?.into_u8()?,
            r#unknown50: row.field(50usize + offset)?.into_u8()?,
            r#damage_phys: row.field(51usize + offset)?.into_u16()?,
            r#damage_mag: row.field(52usize + offset)?.into_u16()?,
            r#delayms: row.field(53usize + offset)?.into_u16()?,
            r#unknown54: row.field(54usize + offset)?.into_u8()?,
            r#block_rate: row.field(55usize + offset)?.into_u16()?,
            r#block: row.field(56usize + offset)?.into_u16()?,
            r#defense_phys: row.field(57usize + offset)?.into_u16()?,
            r#defense_mag: row.field(58usize + offset)?.into_u16()?,
            r#base_param: read_array(
                offset,
                6usize,
                2usize,
                |offset| { Result::Ok(Item_BaseParam::populate(row, offset)?) },
            )?,
            r#item_special_bonus: row.field(71usize + offset)?.into_u8()?,
            r#item_special_bonus_param: row.field(72usize + offset)?.into_u8()?,
            r#base_param_special: read_array(
                offset,
                6usize,
                2usize,
                |offset| { Result::Ok(Item_BaseParamSpecial::populate(row, offset)?) },
            )?,
            r#materialize_type: row.field(85usize + offset)?.into_u8()?,
            r#materia_slot_count: row.field(86usize + offset)?.into_u8()?,
            r#is_advanced_melding_permitted: row.field(87usize + offset)?.into_bool()?,
            r#is_pv_p: row.field(88usize + offset)?.into_bool()?,
            r#sub_stat_category: row.field(89usize + offset)?.into_u8()?,
            r#is_glamourous: row.field(90usize + offset)?.into_bool()?,
        })
    }
}
