use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for MovieStaffList {
    fn name() -> String {
        "MovieStaffList".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MovieStaffList::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MovieStaffList {
    pub r#image: u32,
    pub r#start_time: f32,
    pub r#end_time: f32,
}
impl MovieStaffList {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#image: row.field(0usize + offset)?.into_u32()?,
            r#start_time: row.field(1usize + offset)?.into_f32()?,
            r#end_time: row.field(2usize + offset)?.into_f32()?,
        })
    }
}
