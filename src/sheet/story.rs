use ironworks::sestring::SeString;
use crate::utility::read_array;
use crate::error::PopulateError;
use std::vec::Vec;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Story {
    fn name() -> String {
        "Story".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Story::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Story {
    pub r#script: SeString,
    pub r#instruction: Vec<SeString>,
    pub r#argument: Vec<u32>,
    pub r#sequence: Vec<u16>,
    pub r#completed_quest_operator: Vec<u8>,
    pub r#completed_quest0: Vec<u32>,
    pub r#completed_quest1: Vec<u32>,
    pub r#completed_quest2: Vec<u32>,
    pub r#accepted_quest_operator: Vec<u8>,
    pub r#accepted_quest0: Vec<u32>,
    pub r#accepted_quest_sequence0: Vec<u8>,
    pub r#accepted_quest1: Vec<u32>,
    pub r#accepted_quest_sequence1: Vec<u8>,
    pub r#accepted_quest2: Vec<u32>,
    pub r#accepted_quest_sequence2: Vec<u8>,
    pub r#layer_set0: Vec<u32>,
    pub r#layer_set1: Vec<u32>,
    pub r#sequence_begin: Vec<u16>,
    pub r#sequence_end: Vec<u16>,
    pub r#listener: Vec<u32>,
    pub r#layer_set_territory_type0: u16,
    pub r#layer_set_territory_type1: u16,
}
impl Story {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#script: row.field(0usize + offset)?.into_string()?,
            r#instruction: read_array(
                offset,
                40usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_string()?) },
            )?,
            r#argument: read_array(
                offset,
                40usize,
                1usize,
                |offset| { Result::Ok(row.field(41usize + offset)?.into_u32()?) },
            )?,
            r#sequence: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(81usize + offset)?.into_u16()?) },
            )?,
            r#completed_quest_operator: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(191usize + offset)?.into_u8()?) },
            )?,
            r#completed_quest0: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(301usize + offset)?.into_u32()?) },
            )?,
            r#completed_quest1: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(411usize + offset)?.into_u32()?) },
            )?,
            r#completed_quest2: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(521usize + offset)?.into_u32()?) },
            )?,
            r#accepted_quest_operator: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(631usize + offset)?.into_u8()?) },
            )?,
            r#accepted_quest0: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(741usize + offset)?.into_u32()?) },
            )?,
            r#accepted_quest_sequence0: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(851usize + offset)?.into_u8()?) },
            )?,
            r#accepted_quest1: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(961usize + offset)?.into_u32()?) },
            )?,
            r#accepted_quest_sequence1: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(1071usize + offset)?.into_u8()?) },
            )?,
            r#accepted_quest2: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(1181usize + offset)?.into_u32()?) },
            )?,
            r#accepted_quest_sequence2: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(1291usize + offset)?.into_u8()?) },
            )?,
            r#layer_set0: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(1401usize + offset)?.into_u32()?) },
            )?,
            r#layer_set1: read_array(
                offset,
                110usize,
                1usize,
                |offset| { Result::Ok(row.field(1511usize + offset)?.into_u32()?) },
            )?,
            r#sequence_begin: read_array(
                offset,
                80usize,
                1usize,
                |offset| { Result::Ok(row.field(1621usize + offset)?.into_u16()?) },
            )?,
            r#sequence_end: read_array(
                offset,
                80usize,
                1usize,
                |offset| { Result::Ok(row.field(1701usize + offset)?.into_u16()?) },
            )?,
            r#listener: read_array(
                offset,
                80usize,
                1usize,
                |offset| { Result::Ok(row.field(1781usize + offset)?.into_u32()?) },
            )?,
            r#layer_set_territory_type0: row.field(1861usize + offset)?.into_u16()?,
            r#layer_set_territory_type1: row.field(1862usize + offset)?.into_u16()?,
        })
    }
}
