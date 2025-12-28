use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Cutscene {
    fn name() -> String {
        "Cutscene".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Cutscene::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Cutscene {
    pub r#path: SeString,
    pub r#unknown1: u8,
    pub r#unknown2: u8,
    pub r#unknown3: bool,
    pub r#unknown4: i32,
    pub r#unknown5: i32,
    pub r#unknown6: i32,
    pub r#unknown7: i32,
    pub r#unknown8: u16,
    pub r#unknown9: bool,
}
impl Cutscene {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#path: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_i32()?,
            r#unknown5: row.field(5usize + offset)?.into_i32()?,
            r#unknown6: row.field(6usize + offset)?.into_i32()?,
            r#unknown7: row.field(7usize + offset)?.into_i32()?,
            r#unknown8: row.field(8usize + offset)?.into_u16()?,
            r#unknown9: row.field(9usize + offset)?.into_bool()?,
        })
    }
}
