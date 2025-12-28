use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MovieSubtitle500 {
    fn name() -> String {
        "MovieSubtitle500".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MovieSubtitle500::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MovieSubtitle500 {
    pub r#start_time: f32,
    pub r#end_time: f32,
}
impl MovieSubtitle500 {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#start_time: row.field(0usize + offset)?.into_f32()?,
            r#end_time: row.field(1usize + offset)?.into_f32()?,
        })
    }
}
