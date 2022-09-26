use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for BannerCondition {
    fn name() -> String {
        "BannerCondition".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BannerCondition::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BannerCondition {
    pub r#unlock_type1: u8,
    pub r#unlock_criteria1: u32,
    pub r#unlock_type2: u32,
    pub r#unlock_criteria2: u32,
    pub r#unlock_criteria3: u32,
    pub r#unlock_criteria4: u32,
    pub r#has_prerequisite: u32,
    pub r#prerequisite: u8,
}
impl BannerCondition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unlock_type1: row.field(0usize + offset)?.into_u8()?,
            r#unlock_criteria1: row.field(1usize + offset)?.into_u32()?,
            r#unlock_type2: row.field(2usize + offset)?.into_u32()?,
            r#unlock_criteria2: row.field(3usize + offset)?.into_u32()?,
            r#unlock_criteria3: row.field(4usize + offset)?.into_u32()?,
            r#unlock_criteria4: row.field(5usize + offset)?.into_u32()?,
            r#has_prerequisite: row.field(6usize + offset)?.into_u32()?,
            r#prerequisite: row.field(7usize + offset)?.into_u8()?,
        })
    }
}
