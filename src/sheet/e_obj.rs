use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for EObj {
    fn name() -> String {
        "EObj".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EObj::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EObj {
    pub r#unknown0: bool,
    pub r#unknown1: bool,
    pub r#unknown2: bool,
    pub r#unknown3: bool,
    pub r#unknown4: bool,
    pub r#unknown5: bool,
    pub r#unknown6: bool,
    pub r#unknown7: bool,
    pub r#pop_type: u8,
    pub r#data: u32,
    pub r#invisibility: u8,
    pub r#sgb_path: u16,
    pub r#eye_collision: bool,
    pub r#director_control: bool,
    pub r#target: bool,
    pub r#event_high_addition: u8,
    pub r#unknown16: bool,
    pub r#unknown17: u8,
    pub r#added_in53: bool,
}
impl EObj {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#unknown5: row.field(5usize + offset)?.into_bool()?,
            r#unknown6: row.field(6usize + offset)?.into_bool()?,
            r#unknown7: row.field(7usize + offset)?.into_bool()?,
            r#pop_type: row.field(8usize + offset)?.into_u8()?,
            r#data: row.field(9usize + offset)?.into_u32()?,
            r#invisibility: row.field(10usize + offset)?.into_u8()?,
            r#sgb_path: row.field(11usize + offset)?.into_u16()?,
            r#eye_collision: row.field(12usize + offset)?.into_bool()?,
            r#director_control: row.field(13usize + offset)?.into_bool()?,
            r#target: row.field(14usize + offset)?.into_bool()?,
            r#event_high_addition: row.field(15usize + offset)?.into_u8()?,
            r#unknown16: row.field(16usize + offset)?.into_bool()?,
            r#unknown17: row.field(17usize + offset)?.into_u8()?,
            r#added_in53: row.field(18usize + offset)?.into_bool()?,
        })
    }
}
