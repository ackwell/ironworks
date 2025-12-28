use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#unknown3: i8,
}
impl MovieStaffList {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#image: row.field(0usize + offset)?.into_u32()?,
            r#start_time: row.field(1usize + offset)?.into_f32()?,
            r#end_time: row.field(2usize + offset)?.into_f32()?,
            r#unknown3: row.field(3usize + offset)?.into_i8()?,
        })
    }
}
