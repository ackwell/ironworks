use ironworks::excel::Row;
use crate::utility::read_array;
use std::result::Result;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for RecipeNotebookList {
    fn name() -> String {
        "RecipeNotebookList".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RecipeNotebookList::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RecipeNotebookList {
    pub r#count: u8,
    pub r#recipe: Vec<i32>,
}
impl RecipeNotebookList {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#count: row.field(0usize + offset)?.into_u8()?,
            r#recipe: read_array(
                offset,
                160usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_i32()?) },
            )?,
        })
    }
}
