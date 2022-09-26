use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for MovieSubtitle {
    fn name() -> String {
        "MovieSubtitle".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MovieSubtitle::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MovieSubtitle {
    pub r#start_time: f32,
    pub r#end_time: f32,
}
impl MovieSubtitle {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#start_time: row.field(0usize + offset)?.into_f32()?,
            r#end_time: row.field(1usize + offset)?.into_f32()?,
        })
    }
}
