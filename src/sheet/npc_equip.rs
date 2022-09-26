use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
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
    pub r#model_off_hand: u64,
    pub r#dye_off_hand: u8,
    pub r#model_head: u32,
    pub r#dye_head: u8,
    pub r#visor: bool,
    pub r#model_body: u32,
    pub r#dye_body: u8,
    pub r#model_hands: u32,
    pub r#dye_hands: u8,
    pub r#model_legs: u32,
    pub r#dye_legs: u8,
    pub r#model_feet: u32,
    pub r#dye_feet: u8,
    pub r#model_ears: u32,
    pub r#dye_ears: u8,
    pub r#model_neck: u32,
    pub r#dye_neck: u8,
    pub r#model_wrists: u32,
    pub r#dye_wrists: u8,
    pub r#model_left_ring: u32,
    pub r#dye_left_ring: u8,
    pub r#model_right_ring: u32,
    pub r#dye_right_ring: u8,
}
impl NpcEquip {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#model_main_hand: row.field(0usize + offset)?.into_u64()?,
            r#dye_main_hand: row.field(1usize + offset)?.into_u8()?,
            r#model_off_hand: row.field(2usize + offset)?.into_u64()?,
            r#dye_off_hand: row.field(3usize + offset)?.into_u8()?,
            r#model_head: row.field(4usize + offset)?.into_u32()?,
            r#dye_head: row.field(5usize + offset)?.into_u8()?,
            r#visor: row.field(6usize + offset)?.into_bool()?,
            r#model_body: row.field(7usize + offset)?.into_u32()?,
            r#dye_body: row.field(8usize + offset)?.into_u8()?,
            r#model_hands: row.field(9usize + offset)?.into_u32()?,
            r#dye_hands: row.field(10usize + offset)?.into_u8()?,
            r#model_legs: row.field(11usize + offset)?.into_u32()?,
            r#dye_legs: row.field(12usize + offset)?.into_u8()?,
            r#model_feet: row.field(13usize + offset)?.into_u32()?,
            r#dye_feet: row.field(14usize + offset)?.into_u8()?,
            r#model_ears: row.field(15usize + offset)?.into_u32()?,
            r#dye_ears: row.field(16usize + offset)?.into_u8()?,
            r#model_neck: row.field(17usize + offset)?.into_u32()?,
            r#dye_neck: row.field(18usize + offset)?.into_u8()?,
            r#model_wrists: row.field(19usize + offset)?.into_u32()?,
            r#dye_wrists: row.field(20usize + offset)?.into_u8()?,
            r#model_left_ring: row.field(21usize + offset)?.into_u32()?,
            r#dye_left_ring: row.field(22usize + offset)?.into_u8()?,
            r#model_right_ring: row.field(23usize + offset)?.into_u32()?,
            r#dye_right_ring: row.field(24usize + offset)?.into_u8()?,
        })
    }
}
