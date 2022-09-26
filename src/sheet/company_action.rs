use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
impl MetadataAdapter for CompanyAction {
    fn name() -> String {
        "CompanyAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompanyAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompanyAction {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#icon: i32,
    pub r#fc_rank: u8,
    pub r#cost: u32,
    pub r#order: u8,
    pub r#purchasable: bool,
}
impl CompanyAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#icon: row.field(2usize + offset)?.into_i32()?,
            r#fc_rank: row.field(3usize + offset)?.into_u8()?,
            r#cost: row.field(4usize + offset)?.into_u32()?,
            r#order: row.field(5usize + offset)?.into_u8()?,
            r#purchasable: row.field(6usize + offset)?.into_bool()?,
        })
    }
}
