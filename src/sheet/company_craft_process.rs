use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for CompanyCraftProcess {
    fn name() -> String {
        "CompanyCraftProcess".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompanyCraftProcess::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompanyCraftProcess_St {
    pub r#supply_item: u16,
    pub r#set_quantity: u16,
    pub r#sets_required: u16,
}
impl CompanyCraftProcess_St {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#supply_item: row.field(0usize + offset)?.into_u16()?,
            r#set_quantity: row.field(1usize + offset)?.into_u16()?,
            r#sets_required: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct CompanyCraftProcess {
    pub r#st: Vec<CompanyCraftProcess_St>,
}
impl CompanyCraftProcess {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#st: read_array(
                offset,
                12usize,
                3usize,
                |offset| { Result::Ok(CompanyCraftProcess_St::populate(row, offset)?) },
            )?,
        })
    }
}
