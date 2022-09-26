use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for TripleTriadCardResident {
    fn name() -> String {
        "TripleTriadCardResident".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TripleTriadCardResident::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TripleTriadCardResident {
    pub r#unknown0: u16,
    pub r#top: u8,
    pub r#bottom: u8,
    pub r#left: u8,
    pub r#right: u8,
    pub r#triple_triad_card_rarity: u8,
    pub r#triple_triad_card_type: u8,
    pub r#sale_value: u16,
    pub r#sort_key: u8,
    pub r#order: u16,
    pub r#ui_priority: u8,
    pub r#unknown11: bool,
    pub r#acquisition_type: u8,
    pub r#acquisition: u32,
    pub r#location: u32,
    pub r#quest: u32,
}
impl TripleTriadCardResident {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
            r#top: row.field(1usize + offset)?.into_u8()?,
            r#bottom: row.field(2usize + offset)?.into_u8()?,
            r#left: row.field(3usize + offset)?.into_u8()?,
            r#right: row.field(4usize + offset)?.into_u8()?,
            r#triple_triad_card_rarity: row.field(5usize + offset)?.into_u8()?,
            r#triple_triad_card_type: row.field(6usize + offset)?.into_u8()?,
            r#sale_value: row.field(7usize + offset)?.into_u16()?,
            r#sort_key: row.field(8usize + offset)?.into_u8()?,
            r#order: row.field(9usize + offset)?.into_u16()?,
            r#ui_priority: row.field(10usize + offset)?.into_u8()?,
            r#unknown11: row.field(11usize + offset)?.into_bool()?,
            r#acquisition_type: row.field(12usize + offset)?.into_u8()?,
            r#acquisition: row.field(13usize + offset)?.into_u32()?,
            r#location: row.field(14usize + offset)?.into_u32()?,
            r#quest: row.field(15usize + offset)?.into_u32()?,
        })
    }
}
