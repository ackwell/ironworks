use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for DawnContent {
    fn name() -> String {
        "DawnContent".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DawnContent::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DawnContent {
    pub r#content: u32,
    pub r#exp_below_ex_max_lvl: bool,
    pub r#exp_above_ex_max_lvl: bool,
}
impl DawnContent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#content: row.field(0usize + offset)?.into_u32()?,
            r#exp_below_ex_max_lvl: row.field(1usize + offset)?.into_bool()?,
            r#exp_above_ex_max_lvl: row.field(2usize + offset)?.into_bool()?,
        })
    }
}
