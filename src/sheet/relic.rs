use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for Relic {
    fn name() -> String {
        "Relic".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Relic::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Relic {
    pub r#item_atma: u32,
    pub r#item_animus: u32,
    pub r#icon: i32,
    pub r#materia0: u16,
    pub r#note_main0: u8,
    pub r#note_sub0: u8,
    pub r#note_selection10: u8,
    pub r#materia1: u16,
    pub r#note_main1: u8,
    pub r#note_sub1: u8,
    pub r#note_selection1: u8,
    pub r#materia2: u16,
    pub r#note_main2: u8,
    pub r#note_sub2: u8,
    pub r#materia3: u16,
    pub r#note_selection3: u8,
}
impl Relic {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item_atma: row.field(0usize + offset)?.into_u32()?,
            r#item_animus: row.field(1usize + offset)?.into_u32()?,
            r#icon: row.field(2usize + offset)?.into_i32()?,
            r#materia0: row.field(3usize + offset)?.into_u16()?,
            r#note_main0: row.field(4usize + offset)?.into_u8()?,
            r#note_sub0: row.field(5usize + offset)?.into_u8()?,
            r#note_selection10: row.field(6usize + offset)?.into_u8()?,
            r#materia1: row.field(7usize + offset)?.into_u16()?,
            r#note_main1: row.field(8usize + offset)?.into_u8()?,
            r#note_sub1: row.field(9usize + offset)?.into_u8()?,
            r#note_selection1: row.field(10usize + offset)?.into_u8()?,
            r#materia2: row.field(11usize + offset)?.into_u16()?,
            r#note_main2: row.field(12usize + offset)?.into_u8()?,
            r#note_sub2: row.field(13usize + offset)?.into_u8()?,
            r#materia3: row.field(14usize + offset)?.into_u16()?,
            r#note_selection3: row.field(15usize + offset)?.into_u8()?,
        })
    }
}
