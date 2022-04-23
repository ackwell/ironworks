use std::vec::Vec;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::utility::read_array;
use crate::error::PopulateError;
impl MetadataAdapter for LeveRewardItemGroup {
    fn name() -> String {
        "LeveRewardItemGroup".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(LeveRewardItemGroup::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct LeveRewardItemGroup_Unnamed0 {
    pub r#item: i32,
    pub r#count: u8,
    pub r#hq: bool,
}
impl LeveRewardItemGroup_Unnamed0 {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#count: row.field(1usize + offset)?.into_u8()?,
            r#hq: row.field(2usize + offset)?.into_bool()?,
        })
    }
}
#[derive(Debug)]
pub struct LeveRewardItemGroup {
    pub r#unnamed0: Vec<LeveRewardItemGroup_Unnamed0>,
}
impl LeveRewardItemGroup {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unnamed0: read_array(
                offset,
                9usize,
                3usize,
                |offset| {
                    Result::Ok(LeveRewardItemGroup_Unnamed0::populate(row, offset)?)
                },
            )?,
        })
    }
}
