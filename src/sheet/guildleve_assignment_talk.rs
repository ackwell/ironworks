use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use std::result::Result;
use crate::utility::read_array;
use std::vec::Vec;
impl MetadataAdapter for GuildleveAssignmentTalk {
    fn name() -> String {
        "GuildleveAssignmentTalk".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GuildleveAssignmentTalk::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GuildleveAssignmentTalk {
    pub r#unknown0: bool,
    pub r#unknown1: bool,
    pub r#unknown2: bool,
    pub r#unknown3: bool,
    pub r#unknown4: bool,
    pub r#unknown5: u8,
    pub r#unknown6: u8,
    pub r#unknown7: u8,
    pub r#unknown8: u8,
    pub r#unknown9: u8,
    pub r#unknown10: u32,
    pub r#unknown11: u32,
    pub r#unknown12: u32,
    pub r#unknown13: u32,
    pub r#unknown14: u32,
    pub r#unknown15: i8,
    pub r#unknown16: i8,
    pub r#unknown17: i8,
    pub r#unknown18: i8,
    pub r#unknown19: i8,
    pub r#unknown20: i8,
    pub r#unknown21: i8,
    pub r#unknown22: i8,
    pub r#unknown23: i8,
    pub r#unknown24: i8,
    pub r#unknown25: i32,
    pub r#unknown26: i32,
    pub r#unknown27: i32,
    pub r#unknown28: i32,
    pub r#unknown29: i32,
    pub r#talk: Vec<SeString>,
}
impl GuildleveAssignmentTalk {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#unknown10: row.field(10usize + offset)?.into_u32()?,
            r#unknown11: row.field(11usize + offset)?.into_u32()?,
            r#unknown12: row.field(12usize + offset)?.into_u32()?,
            r#unknown13: row.field(13usize + offset)?.into_u32()?,
            r#unknown14: row.field(14usize + offset)?.into_u32()?,
            r#unknown15: row.field(15usize + offset)?.into_i8()?,
            r#unknown16: row.field(16usize + offset)?.into_i8()?,
            r#unknown17: row.field(17usize + offset)?.into_i8()?,
            r#unknown18: row.field(18usize + offset)?.into_i8()?,
            r#unknown19: row.field(19usize + offset)?.into_i8()?,
            r#unknown20: row.field(20usize + offset)?.into_i8()?,
            r#unknown21: row.field(21usize + offset)?.into_i8()?,
            r#unknown22: row.field(22usize + offset)?.into_i8()?,
            r#unknown23: row.field(23usize + offset)?.into_i8()?,
            r#unknown24: row.field(24usize + offset)?.into_i8()?,
            r#unknown25: row.field(25usize + offset)?.into_i32()?,
            r#unknown26: row.field(26usize + offset)?.into_i32()?,
            r#unknown27: row.field(27usize + offset)?.into_i32()?,
            r#unknown28: row.field(28usize + offset)?.into_i32()?,
            r#unknown29: row.field(29usize + offset)?.into_i32()?,
            r#talk: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(30usize + offset)?.into_string()?) },
            )?,
        })
    }
}
