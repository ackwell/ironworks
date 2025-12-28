use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for TelepoRelay {
    fn name() -> String {
        "TelepoRelay".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TelepoRelay::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TelepoRelay {
    pub r#territory_type_entry: Vec<u16>,
    pub r#territory_type_exit: Vec<u16>,
    pub r#cost: Vec<u16>,
    pub r#unknown24: u16,
    pub r#unknown25: u16,
    pub r#unknown26: u16,
    pub r#unknown27: u32,
}
impl TelepoRelay {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#territory_type_entry: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u16()?) },
            )?,
            r#territory_type_exit: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(8usize + offset)?.into_u16()?) },
            )?,
            r#cost: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(16usize + offset)?.into_u16()?) },
            )?,
            r#unknown24: row.field(24usize + offset)?.into_u16()?,
            r#unknown25: row.field(25usize + offset)?.into_u16()?,
            r#unknown26: row.field(26usize + offset)?.into_u16()?,
            r#unknown27: row.field(27usize + offset)?.into_u32()?,
        })
    }
}
