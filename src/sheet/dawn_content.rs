use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#unknown1: bool,
    pub r#unknown2: bool,
    pub r#unknown3: bool,
    pub r#exp_below_ex_max_lvl: u32,
    pub r#exp_above_ex_max_lvl: u32,
}
impl DawnContent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#content: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#exp_below_ex_max_lvl: row.field(4usize + offset)?.into_u32()?,
            r#exp_above_ex_max_lvl: row.field(5usize + offset)?.into_u32()?,
        })
    }
}
