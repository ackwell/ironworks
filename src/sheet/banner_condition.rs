use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::convert::Infallible;
use std::result::Result;
use std::vec::Vec;
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
    pub r#unlock_criteria1: Vec<u32>,
    pub r#unlock_type2: u32,
    pub r#unlock_criteria2: u8,
    pub r#unlock_criteria3: u32,
    pub r#unlock_criteria4: u8,
    pub r#prerequisite_type: bool,
    pub r#prerequisite: Option<Infallible>,
    pub r#unlock_hint: Option<Infallible>,
}
impl BannerCondition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unlock_type1: row.field(0usize + offset)?.into_u8()?,
            r#unlock_criteria1: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u32()?) },
            )?,
            r#unlock_type2: row.field(7usize + offset)?.into_u32()?,
            r#unlock_criteria2: row.field(8usize + offset)?.into_u8()?,
            r#unlock_criteria3: row.field(9usize + offset)?.into_u32()?,
            r#unlock_criteria4: row.field(10usize + offset)?.into_u8()?,
            r#prerequisite_type: row.field(11usize + offset)?.into_bool()?,
            r#prerequisite: None,
            r#unlock_hint: None,
        })
    }
}
