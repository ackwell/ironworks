use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for BGMSituation {
    fn name() -> String {
        "BGMSituation".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BGMSituation::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BGMSituation {
    pub r#daytime_id: u16,
    pub r#night_id: u16,
    pub r#battle_id: u16,
    pub r#daybreak_id: u16,
    pub r#twilight_id: u16,
}
impl BGMSituation {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#daytime_id: row.field(0usize + offset)?.into_u16()?,
            r#night_id: row.field(1usize + offset)?.into_u16()?,
            r#battle_id: row.field(2usize + offset)?.into_u16()?,
            r#daybreak_id: row.field(3usize + offset)?.into_u16()?,
            r#twilight_id: row.field(4usize + offset)?.into_u16()?,
        })
    }
}
