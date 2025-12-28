use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for TripleTriadRule {
    fn name() -> String {
        "TripleTriadRule".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TripleTriadRule::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TripleTriadRule {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#unknown4: bool,
    pub r#unknown5: u8,
    pub r#unknown6: i32,
}
impl TripleTriadRule {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_i32()?,
        })
    }
}
