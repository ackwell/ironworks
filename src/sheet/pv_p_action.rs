use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use crate::utility::read_array;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for PvPAction {
    fn name() -> String {
        "PvPAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PvPAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PvPAction {
    pub r#action: u16,
    pub r#unknown1: u8,
    pub r#unknown2: u16,
    pub r#unknown3: u16,
    pub r#unknown4: u16,
    pub r#grand_company: Vec<bool>,
}
impl PvPAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#action: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#grand_company: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(5usize + offset)?.into_bool()?) },
            )?,
        })
    }
}
