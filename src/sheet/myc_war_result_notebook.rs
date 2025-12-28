use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for MYCWarResultNotebook {
    fn name() -> String {
        "MYCWarResultNotebook".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MYCWarResultNotebook::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MYCWarResultNotebook {
    pub r#number: u8,
    pub r#unknown1: u8,
    pub r#link: u8,
    pub r#quest: i32,
    pub r#unknown4: i32,
    pub r#icon: i32,
    pub r#image: i32,
    pub r#rarity: u8,
    pub r#name_jp: SeString,
    pub r#name: SeString,
    pub r#description: SeString,
}
impl MYCWarResultNotebook {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#number: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#link: row.field(2usize + offset)?.into_u8()?,
            r#quest: row.field(3usize + offset)?.into_i32()?,
            r#unknown4: row.field(4usize + offset)?.into_i32()?,
            r#icon: row.field(5usize + offset)?.into_i32()?,
            r#image: row.field(6usize + offset)?.into_i32()?,
            r#rarity: row.field(7usize + offset)?.into_u8()?,
            r#name_jp: row.field(8usize + offset)?.into_string()?,
            r#name: row.field(9usize + offset)?.into_string()?,
            r#description: row.field(10usize + offset)?.into_string()?,
        })
    }
}
