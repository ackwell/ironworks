use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for IconLanguage {
    fn name() -> String {
        "IconLanguage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(IconLanguage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct IconLanguage {
    pub r#unknown0: bool,
    pub r#unknown1: bool,
    pub r#unknown2: bool,
    pub r#unknown3: bool,
    pub r#unknown4: bool,
    pub r#unknown5: bool,
    pub r#unknown6: bool,
    pub r#unknown7: bool,
    pub r#unknown8: bool,
    pub r#unknown9: bool,
}
impl IconLanguage {
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
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
            r#unknown9: row.field(9usize + offset)?.into_bool()?,
        })
    }
}
