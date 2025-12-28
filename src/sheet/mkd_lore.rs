use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for MKDLore {
    fn name() -> String {
        "MKDLore".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MKDLore::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MKDLore {
    pub r#unknown0: u32,
    pub r#image: u32,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#unknown4: u32,
    pub r#name: SeString,
    pub r#description: SeString,
}
impl MKDLore {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#image: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
            r#name: row.field(5usize + offset)?.into_string()?,
            r#description: row.field(6usize + offset)?.into_string()?,
        })
    }
}
