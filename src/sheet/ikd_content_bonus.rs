use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for IKDContentBonus {
    fn name() -> String {
        "IKDContentBonus".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(IKDContentBonus::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct IKDContentBonus {
    pub r#objective: SeString,
    pub r#requirement: SeString,
    pub r#unknown2: u16,
    pub r#image: u32,
    pub r#order: u8,
}
impl IKDContentBonus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#objective: row.field(0usize + offset)?.into_string()?,
            r#requirement: row.field(1usize + offset)?.into_string()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#image: row.field(3usize + offset)?.into_u32()?,
            r#order: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
