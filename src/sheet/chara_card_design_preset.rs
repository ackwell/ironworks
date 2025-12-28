use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for CharaCardDesignPreset {
    fn name() -> String {
        "CharaCardDesignPreset".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CharaCardDesignPreset::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CharaCardDesignPreset {
    pub r#base_plate: u16,
    pub r#top_border: u8,
    pub r#bottom_border: u8,
    pub r#backing: u16,
    pub r#pattern_overlay: u16,
    pub r#portrait_frame: u16,
    pub r#plate_frame: u16,
    pub r#accent: u16,
    pub r#sort_key: u16,
    pub r#name: SeString,
}
impl CharaCardDesignPreset {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#base_plate: row.field(0usize + offset)?.into_u16()?,
            r#top_border: row.field(1usize + offset)?.into_u8()?,
            r#bottom_border: row.field(2usize + offset)?.into_u8()?,
            r#backing: row.field(3usize + offset)?.into_u16()?,
            r#pattern_overlay: row.field(4usize + offset)?.into_u16()?,
            r#portrait_frame: row.field(5usize + offset)?.into_u16()?,
            r#plate_frame: row.field(6usize + offset)?.into_u16()?,
            r#accent: row.field(7usize + offset)?.into_u16()?,
            r#sort_key: row.field(8usize + offset)?.into_u16()?,
            r#name: row.field(9usize + offset)?.into_string()?,
        })
    }
}
