use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ScenarioTreeTips {
    fn name() -> String {
        "ScenarioTreeTips".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ScenarioTreeTips::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ScenarioTreeTips {
    pub r#unknown0: u8,
    pub r#tips1: u32,
    pub r#unknown2: u16,
    pub r#tips2: u32,
}
impl ScenarioTreeTips {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#tips1: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#tips2: row.field(3usize + offset)?.into_u32()?,
        })
    }
}
