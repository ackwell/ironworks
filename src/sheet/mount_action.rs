use crate::utility::read_array;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use ironworks::excel::Row;
impl MetadataAdapter for MountAction {
    fn name() -> String {
        "MountAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MountAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MountAction {
    pub r#action: Vec<u16>,
}
impl MountAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#action: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
