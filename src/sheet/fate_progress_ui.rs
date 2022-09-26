use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for FateProgressUI {
    fn name() -> String {
        "FateProgressUI".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FateProgressUI::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FateProgressUI {
    pub r#location: i32,
    pub r#achievement: i32,
    pub r#req_fates_to_rank2: u8,
    pub r#req_fates_to_rank3: u8,
    pub r#unknown4: i8,
    pub r#display_order: u8,
}
impl FateProgressUI {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#location: row.field(0usize + offset)?.into_i32()?,
            r#achievement: row.field(1usize + offset)?.into_i32()?,
            r#req_fates_to_rank2: row.field(2usize + offset)?.into_u8()?,
            r#req_fates_to_rank3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_i8()?,
            r#display_order: row.field(5usize + offset)?.into_u8()?,
        })
    }
}
