use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for TopicSelect {
    fn name() -> String {
        "TopicSelect".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TopicSelect::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TopicSelect_Shop {
    pub r#shop: u32,
}
impl TopicSelect_Shop {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#shop: row.field(4usize + offset)?.into_u32()?,
        })
    }
}
#[derive(Debug)]
pub struct TopicSelect {
    pub r#name: SeString,
    pub r#unknown1: bool,
    pub r#unknown2: u8,
    pub r#unknown3: u16,
    pub r#shop: Vec<TopicSelect_Shop>,
}
impl TopicSelect {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#shop: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(TopicSelect_Shop::populate(row, offset)?) },
            )?,
        })
    }
}
