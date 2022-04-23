use ironworks::excel::Row;
use crate::utility::read_array;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use std::vec::Vec;
impl MetadataAdapter for HowToPage {
    fn name() -> String {
        "HowToPage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HowToPage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HowToPage {
    pub r#type: u8,
    pub r#icon_type: u8,
    pub r#image: i32,
    pub r#text_type: u8,
    pub r#text: Vec<SeString>,
}
impl HowToPage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#icon_type: row.field(1usize + offset)?.into_u8()?,
            r#image: row.field(2usize + offset)?.into_i32()?,
            r#text_type: row.field(3usize + offset)?.into_u8()?,
            r#text: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(4usize + offset)?.into_string()?) },
            )?,
        })
    }
}
