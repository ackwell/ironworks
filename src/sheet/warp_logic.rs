use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::vec::Vec;
use crate::error::PopulateError;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for WarpLogic {
    fn name() -> String {
        "WarpLogic".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WarpLogic::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WarpLogic_Argument {
    pub r#argument: u32,
}
impl WarpLogic_Argument {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#argument: row.field(13usize + offset)?.into_u32()?,
        })
    }
}
#[derive(Debug)]
pub struct WarpLogic {
    pub r#unknown0: u32,
    pub r#warp_name: SeString,
    pub r#can_skip_cutscene: bool,
    pub r#function: Vec<SeString>,
    pub r#argument: Vec<WarpLogic_Argument>,
    pub r#question: SeString,
    pub r#response_yes: SeString,
    pub r#response_no: SeString,
}
impl WarpLogic {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#warp_name: row.field(1usize + offset)?.into_string()?,
            r#can_skip_cutscene: row.field(2usize + offset)?.into_bool()?,
            r#function: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_string()?) },
            )?,
            r#argument: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(WarpLogic_Argument::populate(row, offset)?) },
            )?,
            r#question: row.field(23usize + offset)?.into_string()?,
            r#response_yes: row.field(24usize + offset)?.into_string()?,
            r#response_no: row.field(25usize + offset)?.into_string()?,
        })
    }
}
