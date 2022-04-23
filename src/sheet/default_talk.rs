use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for DefaultTalk {
    fn name() -> String {
        "DefaultTalk".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DefaultTalk::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DefaultTalk {
    pub r#unknown0: u32,
    pub r#unknown1: u8,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#unknown4: u8,
    pub r#action_timeline_pose: Vec<u16>,
    pub r#unknown8: u8,
    pub r#unknown9: u8,
    pub r#unknown10: u8,
    pub r#unknown11: u16,
    pub r#unknown12: u16,
    pub r#unknown13: u16,
    pub r#unknown14: u8,
    pub r#unknown15: u8,
    pub r#unknown16: u8,
    pub r#unknown17: bool,
    pub r#unknown18: bool,
    pub r#unknown19: bool,
    pub r#text: Vec<SeString>,
}
impl DefaultTalk {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
            r#action_timeline_pose: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(5usize + offset)?.into_u16()?) },
            )?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#unknown10: row.field(10usize + offset)?.into_u8()?,
            r#unknown11: row.field(11usize + offset)?.into_u16()?,
            r#unknown12: row.field(12usize + offset)?.into_u16()?,
            r#unknown13: row.field(13usize + offset)?.into_u16()?,
            r#unknown14: row.field(14usize + offset)?.into_u8()?,
            r#unknown15: row.field(15usize + offset)?.into_u8()?,
            r#unknown16: row.field(16usize + offset)?.into_u8()?,
            r#unknown17: row.field(17usize + offset)?.into_bool()?,
            r#unknown18: row.field(18usize + offset)?.into_bool()?,
            r#unknown19: row.field(19usize + offset)?.into_bool()?,
            r#text: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(20usize + offset)?.into_string()?) },
            )?,
        })
    }
}
