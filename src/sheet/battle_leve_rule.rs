use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for BattleLeveRule {
    fn name() -> String {
        "BattleLeveRule".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BattleLeveRule::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BattleLeveRule {
    pub r#rule: SeString,
}
impl BattleLeveRule {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#rule: row.field(0usize + offset)?.into_string()?,
        })
    }
}
