use crate::utility::read_array;
use std::vec::Vec;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for RacingChocoboNameInfo {
    fn name() -> String {
        "RacingChocoboNameInfo".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RacingChocoboNameInfo::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RacingChocoboNameInfo {
    pub r#racing_chocobo_name_category: u8,
    pub r#unknown1: bool,
    pub r#unknown2: bool,
    pub r#unknown3: bool,
    pub r#unknown4: bool,
    pub r#name: Vec<u16>,
}
impl RacingChocoboNameInfo {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#racing_chocobo_name_category: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#name: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(5usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
