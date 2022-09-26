use std::vec::Vec;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::utility::read_array;
use std::result::Result;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ContentsTutorial {
    fn name() -> String {
        "ContentsTutorial".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentsTutorial::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentsTutorial {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#page: Vec<i32>,
}
impl ContentsTutorial {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#page: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_i32()?) },
            )?,
        })
    }
}
