use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for BannerDesignPreset {
    fn name() -> String {
        "BannerDesignPreset".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BannerDesignPreset::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BannerDesignPreset {
    pub r#background: u16,
    pub r#frame: u16,
    pub r#decoration: u16,
    pub r#sort_key: u16,
    pub r#name: SeString,
}
impl BannerDesignPreset {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#background: row.field(0usize + offset)?.into_u16()?,
            r#frame: row.field(1usize + offset)?.into_u16()?,
            r#decoration: row.field(2usize + offset)?.into_u16()?,
            r#sort_key: row.field(3usize + offset)?.into_u16()?,
            r#name: row.field(4usize + offset)?.into_string()?,
        })
    }
}
