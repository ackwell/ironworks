use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Relic3 {
    fn name() -> String {
        "Relic3".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Relic3::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Relic3 {
    pub r#item_animus: u32,
    pub r#item_scroll: u32,
    pub r#materia_limit: u8,
    pub r#item_novus: u32,
    pub r#icon: i32,
}
impl Relic3 {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item_animus: row.field(0usize + offset)?.into_u32()?,
            r#item_scroll: row.field(1usize + offset)?.into_u32()?,
            r#materia_limit: row.field(2usize + offset)?.into_u8()?,
            r#item_novus: row.field(3usize + offset)?.into_u32()?,
            r#icon: row.field(4usize + offset)?.into_i32()?,
        })
    }
}
