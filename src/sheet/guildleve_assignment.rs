use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for GuildleveAssignment {
    fn name() -> String {
        "GuildleveAssignment".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GuildleveAssignment::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GuildleveAssignment {
    pub r#type: SeString,
    pub r#unknown1: u8,
    pub r#assignment_talk: u32,
    pub r#quest: Vec<u32>,
    pub r#unknown5: bool,
    pub r#unknown6: bool,
    pub r#unknown7: bool,
    pub r#unknown8: bool,
    pub r#unknown9: bool,
    pub r#unknown10: u8,
}
impl GuildleveAssignment {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#assignment_talk: row.field(2usize + offset)?.into_u32()?,
            r#quest: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_u32()?) },
            )?,
            r#unknown5: row.field(5usize + offset)?.into_bool()?,
            r#unknown6: row.field(6usize + offset)?.into_bool()?,
            r#unknown7: row.field(7usize + offset)?.into_bool()?,
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
            r#unknown9: row.field(9usize + offset)?.into_bool()?,
            r#unknown10: row.field(10usize + offset)?.into_u8()?,
        })
    }
}
