use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for TripleTriadCardRarity {
    fn name() -> String {
        "TripleTriadCardRarity".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TripleTriadCardRarity::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TripleTriadCardRarity {
    pub r#stars: u8,
}
impl TripleTriadCardRarity {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#stars: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
