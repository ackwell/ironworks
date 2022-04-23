use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for PartyContentCutscene {
    fn name() -> String {
        "PartyContentCutscene".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PartyContentCutscene::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PartyContentCutscene {
    pub r#cutscene: u32,
}
impl PartyContentCutscene {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#cutscene: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
