use std::vec::Vec;
use crate::utility::read_array;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for GuildleveAssignmentCategory {
    fn name() -> String {
        "GuildleveAssignmentCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GuildleveAssignmentCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GuildleveAssignmentCategory {
    pub r#category: Vec<i32>,
}
impl GuildleveAssignmentCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_i32()?) },
            )?,
        })
    }
}
