use std::result::Result;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for TripleTriadCard {
    fn name() -> String {
        "TripleTriadCard".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TripleTriadCard::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TripleTriadCard {
    pub r#name: SeString,
    pub r#unknown1: i8,
    pub r#unknown2: SeString,
    pub r#unknown3: i8,
    pub r#starts_with_vowel: i8,
    pub r#unknown5: i8,
    pub r#unknown6: i8,
    pub r#unknown7: i8,
    pub r#description: SeString,
}
impl TripleTriadCard {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_i8()?,
            r#unknown2: row.field(2usize + offset)?.into_string()?,
            r#unknown3: row.field(3usize + offset)?.into_i8()?,
            r#starts_with_vowel: row.field(4usize + offset)?.into_i8()?,
            r#unknown5: row.field(5usize + offset)?.into_i8()?,
            r#unknown6: row.field(6usize + offset)?.into_i8()?,
            r#unknown7: row.field(7usize + offset)?.into_i8()?,
            r#description: row.field(8usize + offset)?.into_string()?,
        })
    }
}
