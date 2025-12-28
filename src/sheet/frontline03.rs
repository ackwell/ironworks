use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for Frontline03 {
    fn name() -> String {
        "Frontline03".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Frontline03::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Frontline03 {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#unknown4: u8,
    pub r#unknown5: u8,
    pub r#unknown6: u8,
    pub r#unknown7: u8,
    pub r#unknown8: u8,
    pub r#empty_icon: Vec<u32>,
    pub r#maelstrom_icon: Vec<u32>,
    pub r#twin_adder_icon: Vec<u32>,
    pub r#immortal_flames_icon: Vec<u32>,
}
impl Frontline03 {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#empty_icon: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(9usize + offset)?.into_u32()?) },
            )?,
            r#maelstrom_icon: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(12usize + offset)?.into_u32()?) },
            )?,
            r#twin_adder_icon: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(15usize + offset)?.into_u32()?) },
            )?,
            r#immortal_flames_icon: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(18usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
