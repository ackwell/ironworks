use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for CreditList {
    fn name() -> String {
        "CreditList".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CreditList::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CreditList {
    pub r#scale: u16,
    pub r#icon: u32,
    pub r#font: u32,
    pub r#unknown3: u8,
    pub r#unknown4: u8,
    pub r#cast: u32,
}
impl CreditList {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#scale: row.field(0usize + offset)?.into_u16()?,
            r#icon: row.field(1usize + offset)?.into_u32()?,
            r#font: row.field(2usize + offset)?.into_u32()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
            r#cast: row.field(5usize + offset)?.into_u32()?,
        })
    }
}
