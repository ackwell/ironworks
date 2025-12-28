use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Pet {
    fn name() -> String {
        "Pet".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Pet::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Pet {
    pub r#name: SeString,
    pub r#unknown1: u16,
    pub r#unknown2: u16,
    pub r#unknown3: u16,
    pub r#unknown4: u16,
    pub r#unknown5: u16,
    pub r#unknown6: bool,
    pub r#unknown7: bool,
    pub r#unknown8: bool,
    pub r#unknown9: u8,
    pub r#unknown10: u8,
    pub r#unknown11: u8,
    pub r#unknown12: i8,
    pub r#unknown13: u8,
    pub r#unknown14: u8,
    pub r#unknown15: u8,
    pub r#unknown16: u8,
    pub r#unknown17: u8,
    pub r#unknown18: u8,
}
impl Pet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u16()?,
            r#unknown6: row.field(6usize + offset)?.into_bool()?,
            r#unknown7: row.field(7usize + offset)?.into_bool()?,
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#unknown10: row.field(10usize + offset)?.into_u8()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_i8()?,
            r#unknown13: row.field(13usize + offset)?.into_u8()?,
            r#unknown14: row.field(14usize + offset)?.into_u8()?,
            r#unknown15: row.field(15usize + offset)?.into_u8()?,
            r#unknown16: row.field(16usize + offset)?.into_u8()?,
            r#unknown17: row.field(17usize + offset)?.into_u8()?,
            r#unknown18: row.field(18usize + offset)?.into_u8()?,
        })
    }
}
