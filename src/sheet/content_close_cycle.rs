use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ContentCloseCycle {
    fn name() -> String {
        "ContentCloseCycle".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentCloseCycle::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentCloseCycle {
    pub r#unixtime: u32,
    pub r#time_seconds: u32,
    pub r#unknown2: u32,
    pub r#unknown3: bool,
    pub r#unknown4: bool,
    pub r#unknown5: bool,
    pub r#unknown6: bool,
    pub r#unknown7: bool,
    pub r#unknown8: bool,
    pub r#unknown9: bool,
    pub r#unknown10: bool,
    pub r#unknown11: bool,
    pub r#unknown12: bool,
}
impl ContentCloseCycle {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unixtime: row.field(0usize + offset)?.into_u32()?,
            r#time_seconds: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u32()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#unknown5: row.field(5usize + offset)?.into_bool()?,
            r#unknown6: row.field(6usize + offset)?.into_bool()?,
            r#unknown7: row.field(7usize + offset)?.into_bool()?,
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
            r#unknown9: row.field(9usize + offset)?.into_bool()?,
            r#unknown10: row.field(10usize + offset)?.into_bool()?,
            r#unknown11: row.field(11usize + offset)?.into_bool()?,
            r#unknown12: row.field(12usize + offset)?.into_bool()?,
        })
    }
}
