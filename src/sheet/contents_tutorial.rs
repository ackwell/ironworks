use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
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
    pub r#page: Vec<i32>,
    pub r#name: u32,
    pub r#description: SeString,
    pub r#unknown10: SeString,
}
impl ContentsTutorial {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#page: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_i32()?) },
            )?,
            r#name: row.field(8usize + offset)?.into_u32()?,
            r#description: row.field(9usize + offset)?.into_string()?,
            r#unknown10: row.field(10usize + offset)?.into_string()?,
        })
    }
}
