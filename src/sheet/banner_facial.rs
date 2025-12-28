use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#unknown2: u16,
    pub r#unknown3: u16,
    pub r#sort_key: u16,
    pub r#unknown5: u8,
}
impl BannerFacial {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#emote: row.field(0usize + offset)?.into_u16()?,
            r#unlock_condition: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#sort_key: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
        })
    }
}
