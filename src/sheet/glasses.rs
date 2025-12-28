use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Glasses {
    fn name() -> String {
        "Glasses".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Glasses::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Glasses {
    pub r#unknown0: u32,
    pub r#glasses_style: i16,
    pub r#icon: i32,
    pub r#unknown3: u16,
    pub r#singular: SeString,
    pub r#unknown5: i8,
    pub r#plural: SeString,
    pub r#unknown7: i8,
    pub r#unknown8: i8,
    pub r#unknown9: i8,
    pub r#unknown10: i8,
    pub r#unknown11: i8,
    pub r#description: SeString,
    pub r#name: SeString,
}
impl Glasses {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#glasses_style: row.field(1usize + offset)?.into_i16()?,
            r#icon: row.field(2usize + offset)?.into_i32()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#singular: row.field(4usize + offset)?.into_string()?,
            r#unknown5: row.field(5usize + offset)?.into_i8()?,
            r#plural: row.field(6usize + offset)?.into_string()?,
            r#unknown7: row.field(7usize + offset)?.into_i8()?,
            r#unknown8: row.field(8usize + offset)?.into_i8()?,
            r#unknown9: row.field(9usize + offset)?.into_i8()?,
            r#unknown10: row.field(10usize + offset)?.into_i8()?,
            r#unknown11: row.field(11usize + offset)?.into_i8()?,
            r#description: row.field(12usize + offset)?.into_string()?,
            r#name: row.field(13usize + offset)?.into_string()?,
        })
    }
}
