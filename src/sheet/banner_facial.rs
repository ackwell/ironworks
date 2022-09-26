use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for BannerFacial {
    fn name() -> String {
        "BannerFacial".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BannerFacial::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BannerFacial {
    pub r#emote: u16,
    pub r#unlock_condition: u16,
    pub r#sort_key: u8,
}
impl BannerFacial {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#emote: row.field(0usize + offset)?.into_u16()?,
            r#unlock_condition: row.field(1usize + offset)?.into_u16()?,
            r#sort_key: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
