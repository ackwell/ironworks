use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for AkatsukiNote {
    fn name() -> String {
        "AkatsukiNote".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AkatsukiNote::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AkatsukiNote {
    pub r#unknown0: i32,
    pub r#unknown1: i32,
    pub r#unknown2: i32,
    pub r#unknown3: i32,
    pub r#unknown4: i32,
    pub r#unknown5: i32,
    pub r#unknown6: i32,
    pub r#unknown7: i32,
    pub r#unknown8: i32,
}
impl AkatsukiNote {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_i32()?,
            r#unknown2: row.field(2usize + offset)?.into_i32()?,
            r#unknown3: row.field(3usize + offset)?.into_i32()?,
            r#unknown4: row.field(4usize + offset)?.into_i32()?,
            r#unknown5: row.field(5usize + offset)?.into_i32()?,
            r#unknown6: row.field(6usize + offset)?.into_i32()?,
            r#unknown7: row.field(7usize + offset)?.into_i32()?,
            r#unknown8: row.field(8usize + offset)?.into_i32()?,
        })
    }
}
