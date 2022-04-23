use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MinionRules {
    fn name() -> String {
        "MinionRules".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MinionRules::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MinionRules {
    pub r#rule: SeString,
    pub r#description: SeString,
}
impl MinionRules {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#rule: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
        })
    }
}
