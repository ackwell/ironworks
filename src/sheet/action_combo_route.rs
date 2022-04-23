use ironworks::sestring::SeString;
use crate::utility::read_array;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ActionComboRoute {
    fn name() -> String {
        "ActionComboRoute".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActionComboRoute::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActionComboRoute {
    pub r#name: SeString,
    pub r#unknown1: i8,
    pub r#action: Vec<u16>,
}
impl ActionComboRoute {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_i8()?,
            r#action: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
