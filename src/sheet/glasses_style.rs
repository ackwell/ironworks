use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for GlassesStyle {
    fn name() -> String {
        "GlassesStyle".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GlassesStyle::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GlassesStyle {
    pub r#unknown0: i16,
    pub r#icon: i32,
    pub r#order: u16,
    pub r#glasses: Vec<u16>,
    pub r#singular: SeString,
    pub r#unknown16: i8,
    pub r#plural: SeString,
    pub r#unknown18: i8,
    pub r#unknown19: i8,
    pub r#unknown20: i8,
    pub r#unknown21: i8,
    pub r#unknown22: i8,
    pub r#name: SeString,
}
impl GlassesStyle {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i16()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#order: row.field(2usize + offset)?.into_u16()?,
            r#glasses: read_array(
                offset,
                12usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_u16()?) },
            )?,
            r#singular: row.field(15usize + offset)?.into_string()?,
            r#unknown16: row.field(16usize + offset)?.into_i8()?,
            r#plural: row.field(17usize + offset)?.into_string()?,
            r#unknown18: row.field(18usize + offset)?.into_i8()?,
            r#unknown19: row.field(19usize + offset)?.into_i8()?,
            r#unknown20: row.field(20usize + offset)?.into_i8()?,
            r#unknown21: row.field(21usize + offset)?.into_i8()?,
            r#unknown22: row.field(22usize + offset)?.into_i8()?,
            r#name: row.field(23usize + offset)?.into_string()?,
        })
    }
}
