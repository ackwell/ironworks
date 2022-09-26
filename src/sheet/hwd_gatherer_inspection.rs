use crate::metadata::MetadataAdapter;
use std::result::Result;
use std::vec::Vec;
use crate::utility::read_array;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for HWDGathererInspection {
    fn name() -> String {
        "HWDGathererInspection".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDGathererInspection::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDGathererInspection {
    pub r#item_required: Vec<u32>,
    pub r#fish_parameter: Vec<u32>,
    pub r#amount_required: Vec<u8>,
    pub r#item_received: Vec<u32>,
    pub r#reward1: Vec<u16>,
    pub r#reward2: Vec<u16>,
    pub r#phase: Vec<u8>,
}
impl HWDGathererInspection {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item_required: read_array(
                offset,
                79usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
            r#fish_parameter: read_array(
                offset,
                79usize,
                1usize,
                |offset| { Result::Ok(row.field(79usize + offset)?.into_u32()?) },
            )?,
            r#amount_required: read_array(
                offset,
                79usize,
                1usize,
                |offset| { Result::Ok(row.field(158usize + offset)?.into_u8()?) },
            )?,
            r#item_received: read_array(
                offset,
                79usize,
                1usize,
                |offset| { Result::Ok(row.field(237usize + offset)?.into_u32()?) },
            )?,
            r#reward1: read_array(
                offset,
                79usize,
                1usize,
                |offset| { Result::Ok(row.field(316usize + offset)?.into_u16()?) },
            )?,
            r#reward2: read_array(
                offset,
                79usize,
                1usize,
                |offset| { Result::Ok(row.field(395usize + offset)?.into_u16()?) },
            )?,
            r#phase: read_array(
                offset,
                79usize,
                1usize,
                |offset| { Result::Ok(row.field(474usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
