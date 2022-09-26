use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for MJILivelyActor {
    fn name() -> String {
        "MJILivelyActor".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJILivelyActor::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJILivelyActor {
    pub r#enpc: u32,
    pub r#behavior: u16,
    pub r#x: f32,
    pub r#y: f32,
    pub r#z: f32,
    pub r#rot: f32,
}
impl MJILivelyActor {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#enpc: row.field(0usize + offset)?.into_u32()?,
            r#behavior: row.field(1usize + offset)?.into_u16()?,
            r#x: row.field(2usize + offset)?.into_f32()?,
            r#y: row.field(3usize + offset)?.into_f32()?,
            r#z: row.field(4usize + offset)?.into_f32()?,
            r#rot: row.field(5usize + offset)?.into_f32()?,
        })
    }
}
