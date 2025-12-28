use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for UIColor {
    fn name() -> String {
        "UIColor".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(UIColor::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct UIColor {
    pub r#ui_foreground: u32,
    pub r#ui_glow: u32,
    pub r#unknown2: u32,
    pub r#unknown3: u32,
    pub r#unknown4: u32,
    pub r#unknown5: u32,
    pub r#unknown6: u32,
    pub r#unknown7: u32,
}
impl UIColor {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#ui_foreground: row.field(0usize + offset)?.into_u32()?,
            r#ui_glow: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u32()?,
            r#unknown3: row.field(3usize + offset)?.into_u32()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
            r#unknown5: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
        })
    }
}
