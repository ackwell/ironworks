use ironworks::excel::Row;
use std::result::Result;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use crate::error::PopulateError;
impl MetadataAdapter for GatheringNotebookList {
    fn name() -> String {
        "GatheringNotebookList".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringNotebookList::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringNotebookList {
    pub r#unknown0: u8,
    pub r#gathering_item: Vec<i32>,
}
impl GatheringNotebookList {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#gathering_item: read_array(
                offset,
                100usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_i32()?) },
            )?,
        })
    }
}
