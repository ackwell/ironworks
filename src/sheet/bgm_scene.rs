use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for BGMScene {
    fn name() -> String {
        "BGMScene".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BGMScene::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BGMScene {
    pub r#enable_disable_restart: bool,
    pub r#resume: bool,
    pub r#enable_pass_end: bool,
    pub r#force_auto_reset: bool,
    pub r#ignore_battle: bool,
}
impl BGMScene {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#enable_disable_restart: row.field(0usize + offset)?.into_bool()?,
            r#resume: row.field(1usize + offset)?.into_bool()?,
            r#enable_pass_end: row.field(2usize + offset)?.into_bool()?,
            r#force_auto_reset: row.field(3usize + offset)?.into_bool()?,
            r#ignore_battle: row.field(4usize + offset)?.into_bool()?,
        })
    }
}
