use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Tribe {
    fn name() -> String {
        "Tribe".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Tribe::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Tribe {
    pub r#masculine: SeString,
    pub r#feminine: SeString,
    pub r#hp: i8,
    pub r#mp: i8,
    pub r#str: i8,
    pub r#vit: i8,
    pub r#dex: i8,
    pub r#int: i8,
    pub r#mnd: i8,
    pub r#pie: i8,
}
impl Tribe {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#masculine: row.field(0usize + offset)?.into_string()?,
            r#feminine: row.field(1usize + offset)?.into_string()?,
            r#hp: row.field(2usize + offset)?.into_i8()?,
            r#mp: row.field(3usize + offset)?.into_i8()?,
            r#str: row.field(4usize + offset)?.into_i8()?,
            r#vit: row.field(5usize + offset)?.into_i8()?,
            r#dex: row.field(6usize + offset)?.into_i8()?,
            r#int: row.field(7usize + offset)?.into_i8()?,
            r#mnd: row.field(8usize + offset)?.into_i8()?,
            r#pie: row.field(9usize + offset)?.into_i8()?,
        })
    }
}
