use crate::error::PopulateError;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for SecretRecipeBook {
    fn name() -> String {
        "SecretRecipeBook".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SecretRecipeBook::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SecretRecipeBook {
    pub r#item: i32,
    pub r#name: SeString,
}
impl SecretRecipeBook {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#name: row.field(1usize + offset)?.into_string()?,
        })
    }
}
