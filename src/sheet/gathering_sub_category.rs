use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for GatheringSubCategory {
    fn name() -> String {
        "GatheringSubCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringSubCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringSubCategory {
    pub r#gathering_type: u8,
    pub r#class_job: u8,
    pub r#quest: u32,
    pub r#division: u16,
    pub r#item: i32,
    pub r#folklore_book: SeString,
    pub r#unknown6: u8,
}
impl GatheringSubCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#gathering_type: row.field(0usize + offset)?.into_u8()?,
            r#class_job: row.field(1usize + offset)?.into_u8()?,
            r#quest: row.field(2usize + offset)?.into_u32()?,
            r#division: row.field(3usize + offset)?.into_u16()?,
            r#item: row.field(4usize + offset)?.into_i32()?,
            r#folklore_book: row.field(5usize + offset)?.into_string()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
        })
    }
}
