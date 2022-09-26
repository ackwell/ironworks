use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
impl MetadataAdapter for MinionRace {
    fn name() -> String {
        "MinionRace".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MinionRace::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MinionRace {
    pub r#name: SeString,
}
impl MinionRace {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
