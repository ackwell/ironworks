use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for NpcEquip {
    fn name() -> String {
        "NpcEquip".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(NpcEquip::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct NpcEquip {
    pub r#model_main_hand: u64,
    pub r#dye_main_hand: u8,
    pub r#dye2_main_hand: u8,
    pub r#model_off_hand: u64,
    pub r#dye_off_hand: u8,
    pub r#dye2_off_hand: u8,
    pub r#model_head: u32,
    pub r#dye_head: u8,
    pub r#dye2_head: u8,
    pub r#visor: bool,
    pub r#model_body: bool,
    pub r#dye_body: u32,
    pub r#dye2_body: u8,
    pub r#model_hands: u8,
    pub r#dye_hands: u32,
    pub r#dye2_hands: u8,
    pub r#model_legs: u8,
    pub r#dye_legs: u32,
    pub r#dye2_legs: u8,
    pub r#model_feet: u8,
    pub r#dye_feet: u32,
    pub r#dye2_feet: u8,
    pub r#model_ears: u8,
    pub r#dye_ears: u32,
    pub r#dye2_ears: u8,
    pub r#model_neck: u8,
    pub r#dye_neck: u32,
    pub r#dye2_neck: u8,
    pub r#model_wrists: u8,
    pub r#dye_wrists: u32,
    pub r#dye2_wrists: u8,
    pub r#model_left_ring: u8,
    pub r#dye_left_ring: u32,
    pub r#dye2_left_ring: u8,
    pub r#model_right_ring: u8,
    pub r#dye_right_ring: u32,
    pub r#dye2_right_ring: u8,
    pub r#unknown37: u8,
    pub r#unknown38: u16,
    pub r#unknown39: u16,
}
impl NpcEquip {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#model_main_hand: row.field(0usize + offset)?.into_u64()?,
            r#dye_main_hand: row.field(1usize + offset)?.into_u8()?,
            r#dye2_main_hand: row.field(2usize + offset)?.into_u8()?,
            r#model_off_hand: row.field(3usize + offset)?.into_u64()?,
            r#dye_off_hand: row.field(4usize + offset)?.into_u8()?,
            r#dye2_off_hand: row.field(5usize + offset)?.into_u8()?,
            r#model_head: row.field(6usize + offset)?.into_u32()?,
            r#dye_head: row.field(7usize + offset)?.into_u8()?,
            r#dye2_head: row.field(8usize + offset)?.into_u8()?,
            r#visor: row.field(9usize + offset)?.into_bool()?,
            r#model_body: row.field(10usize + offset)?.into_bool()?,
            r#dye_body: row.field(11usize + offset)?.into_u32()?,
            r#dye2_body: row.field(12usize + offset)?.into_u8()?,
            r#model_hands: row.field(13usize + offset)?.into_u8()?,
            r#dye_hands: row.field(14usize + offset)?.into_u32()?,
            r#dye2_hands: row.field(15usize + offset)?.into_u8()?,
            r#model_legs: row.field(16usize + offset)?.into_u8()?,
            r#dye_legs: row.field(17usize + offset)?.into_u32()?,
            r#dye2_legs: row.field(18usize + offset)?.into_u8()?,
            r#model_feet: row.field(19usize + offset)?.into_u8()?,
            r#dye_feet: row.field(20usize + offset)?.into_u32()?,
            r#dye2_feet: row.field(21usize + offset)?.into_u8()?,
            r#model_ears: row.field(22usize + offset)?.into_u8()?,
            r#dye_ears: row.field(23usize + offset)?.into_u32()?,
            r#dye2_ears: row.field(24usize + offset)?.into_u8()?,
            r#model_neck: row.field(25usize + offset)?.into_u8()?,
            r#dye_neck: row.field(26usize + offset)?.into_u32()?,
            r#dye2_neck: row.field(27usize + offset)?.into_u8()?,
            r#model_wrists: row.field(28usize + offset)?.into_u8()?,
            r#dye_wrists: row.field(29usize + offset)?.into_u32()?,
            r#dye2_wrists: row.field(30usize + offset)?.into_u8()?,
            r#model_left_ring: row.field(31usize + offset)?.into_u8()?,
            r#dye_left_ring: row.field(32usize + offset)?.into_u32()?,
            r#dye2_left_ring: row.field(33usize + offset)?.into_u8()?,
            r#model_right_ring: row.field(34usize + offset)?.into_u8()?,
            r#dye_right_ring: row.field(35usize + offset)?.into_u32()?,
            r#dye2_right_ring: row.field(36usize + offset)?.into_u8()?,
            r#unknown37: row.field(37usize + offset)?.into_u8()?,
            r#unknown38: row.field(38usize + offset)?.into_u16()?,
            r#unknown39: row.field(39usize + offset)?.into_u16()?,
        })
    }
}
