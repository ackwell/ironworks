use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for BenchmarkOverrideEquipment {
    fn name() -> String {
        "BenchmarkOverrideEquipment".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BenchmarkOverrideEquipment::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BenchmarkOverrideEquipment {
    pub r#unknown0: u32,
    pub r#unknown1: u32,
    pub r#unknown2: u8,
    pub r#unknown3: i8,
    pub r#model_main_hand: u64,
    pub r#dye_main_hand: u8,
    pub r#model_off_hand: u64,
    pub r#dye_off_hand: u8,
    pub r#unknown8: u64,
    pub r#unknown9: u8,
    pub r#model_head: u32,
    pub r#dye_head: u8,
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
impl BenchmarkOverrideEquipment {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_i8()?,
            r#model_main_hand: row.field(4usize + offset)?.into_u64()?,
            r#dye_main_hand: row.field(5usize + offset)?.into_u8()?,
            r#model_off_hand: row.field(6usize + offset)?.into_u64()?,
            r#dye_off_hand: row.field(7usize + offset)?.into_u8()?,
            r#unknown8: row.field(8usize + offset)?.into_u64()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#model_head: row.field(10usize + offset)?.into_u32()?,
            r#dye_head: row.field(11usize + offset)?.into_u8()?,
            r#model_body: row.field(12usize + offset)?.into_u32()?,
            r#dye_body: row.field(13usize + offset)?.into_u8()?,
            r#model_hands: row.field(14usize + offset)?.into_u32()?,
            r#dye_hands: row.field(15usize + offset)?.into_u8()?,
            r#model_legs: row.field(16usize + offset)?.into_u32()?,
            r#dye_legs: row.field(17usize + offset)?.into_u8()?,
            r#model_feet: row.field(18usize + offset)?.into_u32()?,
            r#dye_feet: row.field(19usize + offset)?.into_u8()?,
            r#model_ears: row.field(20usize + offset)?.into_u32()?,
            r#dye_ears: row.field(21usize + offset)?.into_u8()?,
            r#model_neck: row.field(22usize + offset)?.into_u32()?,
            r#dye_neck: row.field(23usize + offset)?.into_u8()?,
            r#model_wrists: row.field(24usize + offset)?.into_u32()?,
            r#dye_wrists: row.field(25usize + offset)?.into_u8()?,
            r#model_left_ring: row.field(26usize + offset)?.into_u32()?,
            r#dye_left_ring: row.field(27usize + offset)?.into_u8()?,
            r#model_right_ring: row.field(28usize + offset)?.into_u32()?,
            r#dye_right_ring: row.field(29usize + offset)?.into_u8()?,
        })
    }
}
