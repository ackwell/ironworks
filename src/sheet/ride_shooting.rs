use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::utility::read_array;
impl MetadataAdapter for RideShooting {
    fn name() -> String {
        "RideShooting".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RideShooting::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RideShooting {
    pub r#g_fate_ride_shooting: u16,
    pub r#unknown1: i16,
    pub r#unknown2: i16,
    pub r#unknown3: u16,
    pub r#unknown4: u16,
    pub r#start_text: u16,
    pub r#unknown6: u32,
    pub r#unknown7: u32,
    pub r#unknown8: u32,
    pub r#unknown9: u32,
    pub r#unknown10: u32,
    pub r#unknown11: u32,
    pub r#unknown12: u32,
    pub r#unknown13: u32,
    pub r#pop_range: Vec<u32>,
    pub r#e_npc: Vec<u32>,
    pub r#e_npc_scale: Vec<u8>,
}
impl RideShooting {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#g_fate_ride_shooting: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_i16()?,
            r#unknown2: row.field(2usize + offset)?.into_i16()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#start_text: row.field(5usize + offset)?.into_u16()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
            r#unknown8: row.field(8usize + offset)?.into_u32()?,
            r#unknown9: row.field(9usize + offset)?.into_u32()?,
            r#unknown10: row.field(10usize + offset)?.into_u32()?,
            r#unknown11: row.field(11usize + offset)?.into_u32()?,
            r#unknown12: row.field(12usize + offset)?.into_u32()?,
            r#unknown13: row.field(13usize + offset)?.into_u32()?,
            r#pop_range: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(14usize + offset)?.into_u32()?) },
            )?,
            r#e_npc: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(22usize + offset)?.into_u32()?) },
            )?,
            r#e_npc_scale: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(30usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
