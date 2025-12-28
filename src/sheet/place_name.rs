use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for PlaceName {
    fn name() -> String {
        "PlaceName".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PlaceName::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PlaceName {
    pub r#name: SeString,
    pub r#unknown1: i8,
    pub r#name_no_article: SeString,
    pub r#unknown3: i8,
    pub r#unknown4: i8,
    pub r#unknown5: i8,
    pub r#unknown6: i8,
    pub r#unknown7: i8,
    pub r#unknown8: SeString,
    pub r#unknown9: u8,
    pub r#unknown10: u16,
    pub r#unknown11: u8,
}
impl PlaceName {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_i8()?,
            r#name_no_article: row.field(2usize + offset)?.into_string()?,
            r#unknown3: row.field(3usize + offset)?.into_i8()?,
            r#unknown4: row.field(4usize + offset)?.into_i8()?,
            r#unknown5: row.field(5usize + offset)?.into_i8()?,
            r#unknown6: row.field(6usize + offset)?.into_i8()?,
            r#unknown7: row.field(7usize + offset)?.into_i8()?,
            r#unknown8: row.field(8usize + offset)?.into_string()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#unknown10: row.field(10usize + offset)?.into_u16()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
        })
    }
}
