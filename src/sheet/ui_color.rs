use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
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
}
impl UIColor {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#ui_foreground: row.field(0usize + offset)?.into_u32()?,
            r#ui_glow: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
