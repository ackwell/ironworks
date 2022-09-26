use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::vec::Vec;
use crate::utility::read_array;
impl MetadataAdapter for DescriptionPage {
    fn name() -> String {
        "DescriptionPage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DescriptionPage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DescriptionPage_e {
    pub r#text: u16,
    pub r#image: u32,
}
impl DescriptionPage_e {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(3usize + offset)?.into_u16()?,
            r#image: row.field(4usize + offset)?.into_u32()?,
        })
    }
}
#[derive(Debug)]
pub struct DescriptionPage {
    pub r#unknown0: u8,
    pub r#quest: u32,
    pub r#unknown2: u8,
    pub r#e: Vec<DescriptionPage_e>,
}
impl DescriptionPage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#quest: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#e: read_array(
                offset,
                11usize,
                2usize,
                |offset| { Result::Ok(DescriptionPage_e::populate(row, offset)?) },
            )?,
        })
    }
}
