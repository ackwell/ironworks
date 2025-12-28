use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Stain {
    fn name() -> String {
        "Stain".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Stain::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Stain {
    pub r#color: u32,
    pub r#shade: u8,
    pub r#sub_order: u8,
    pub r#name: SeString,
    pub r#name2: SeString,
    pub r#unknown5: bool,
    pub r#unknown6: bool,
}
impl Stain {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#color: row.field(0usize + offset)?.into_u32()?,
            r#shade: row.field(1usize + offset)?.into_u8()?,
            r#sub_order: row.field(2usize + offset)?.into_u8()?,
            r#name: row.field(3usize + offset)?.into_string()?,
            r#name2: row.field(4usize + offset)?.into_string()?,
            r#unknown5: row.field(5usize + offset)?.into_bool()?,
            r#unknown6: row.field(6usize + offset)?.into_bool()?,
        })
    }
}
