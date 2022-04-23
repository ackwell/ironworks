use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
impl MetadataAdapter for WeddingBGM {
    fn name() -> String {
        "WeddingBGM".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WeddingBGM::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WeddingBGM {
    pub r#song: u16,
    pub r#song_name: SeString,
}
impl WeddingBGM {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#song: row.field(0usize + offset)?.into_u16()?,
            r#song_name: row.field(1usize + offset)?.into_string()?,
        })
    }
}
