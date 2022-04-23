use ironworks::excel::Row;
use std::result::Result;
use std::convert::Infallible;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for PublicContent {
    fn name() -> String {
        "PublicContent".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PublicContent::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PublicContent {
    pub r#type: u8,
    pub r#time_limit: u16,
    pub r#map_icon: u32,
    pub r#name: SeString,
    pub r#text_data_start: u32,
    pub r#text_data_end: u32,
    pub r#start_cutscene: u32,
    pub r#lgb_event_range: u32,
    pub r#lgb_pop_range: u32,
    pub r#unknown9: u16,
    pub r#unknown10: u16,
    pub r#unknown11: u8,
    pub r#unknown12: u16,
    pub r#unknown13: u16,
    pub r#unknown14: u16,
    pub r#unknown15: u16,
    pub r#end_cutscene: u32,
    pub r#content_finder_condition: Option<Infallible>,
    pub r#additional_data: Option<Infallible>,
}
impl PublicContent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#time_limit: row.field(1usize + offset)?.into_u16()?,
            r#map_icon: row.field(2usize + offset)?.into_u32()?,
            r#name: row.field(3usize + offset)?.into_string()?,
            r#text_data_start: row.field(4usize + offset)?.into_u32()?,
            r#text_data_end: row.field(5usize + offset)?.into_u32()?,
            r#start_cutscene: row.field(6usize + offset)?.into_u32()?,
            r#lgb_event_range: row.field(7usize + offset)?.into_u32()?,
            r#lgb_pop_range: row.field(8usize + offset)?.into_u32()?,
            r#unknown9: row.field(9usize + offset)?.into_u16()?,
            r#unknown10: row.field(10usize + offset)?.into_u16()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_u16()?,
            r#unknown13: row.field(13usize + offset)?.into_u16()?,
            r#unknown14: row.field(14usize + offset)?.into_u16()?,
            r#unknown15: row.field(15usize + offset)?.into_u16()?,
            r#end_cutscene: row.field(16usize + offset)?.into_u32()?,
            r#content_finder_condition: None,
            r#additional_data: None,
        })
    }
}
