use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Credit {
    fn name() -> String {
        "Credit".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Credit::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Credit {
    pub r#unknown0: u8,
    pub r#roles1: u16,
    pub r#japanese_cast1: u16,
    pub r#english_cast1: u16,
    pub r#french_cast1: u16,
    pub r#german_cast1: u16,
    pub r#roles2: u16,
    pub r#japanese_cast2: u16,
    pub r#english_cast2: u16,
    pub r#french_cast2: u16,
    pub r#german_cast2: u16,
}
impl Credit {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#roles1: row.field(1usize + offset)?.into_u16()?,
            r#japanese_cast1: row.field(2usize + offset)?.into_u16()?,
            r#english_cast1: row.field(3usize + offset)?.into_u16()?,
            r#french_cast1: row.field(4usize + offset)?.into_u16()?,
            r#german_cast1: row.field(5usize + offset)?.into_u16()?,
            r#roles2: row.field(6usize + offset)?.into_u16()?,
            r#japanese_cast2: row.field(7usize + offset)?.into_u16()?,
            r#english_cast2: row.field(8usize + offset)?.into_u16()?,
            r#french_cast2: row.field(9usize + offset)?.into_u16()?,
            r#german_cast2: row.field(10usize + offset)?.into_u16()?,
        })
    }
}
