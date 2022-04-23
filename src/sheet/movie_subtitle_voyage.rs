use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for MovieSubtitleVoyage {
    fn name() -> String {
        "MovieSubtitleVoyage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MovieSubtitleVoyage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MovieSubtitleVoyage {
    pub r#start_time: f32,
    pub r#end_time: f32,
}
impl MovieSubtitleVoyage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#start_time: row.field(0usize + offset)?.into_f32()?,
            r#end_time: row.field(1usize + offset)?.into_f32()?,
        })
    }
}
