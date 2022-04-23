use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use std::vec::Vec;
use crate::error::PopulateError;
impl MetadataAdapter for RetainerTaskParameter {
    fn name() -> String {
        "RetainerTaskParameter".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RetainerTaskParameter::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RetainerTaskParameter {
    pub r#item_level_do_w: Vec<i16>,
    pub r#perception_do_l: Vec<i16>,
    pub r#perception_fsh: Vec<i16>,
}
impl RetainerTaskParameter {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item_level_do_w: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_i16()?) },
            )?,
            r#perception_do_l: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(4usize + offset)?.into_i16()?) },
            )?,
            r#perception_fsh: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(8usize + offset)?.into_i16()?) },
            )?,
        })
    }
}
