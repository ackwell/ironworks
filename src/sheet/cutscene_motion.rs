use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for CutsceneMotion {
    fn name() -> String {
        "CutsceneMotion".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CutsceneMotion::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CutsceneMotion {
    pub r#walkloopspeed: f32,
    pub r#runloopspeed: f32,
    pub r#slowwalkloopspeed: f32,
    pub r#slowrunloopspeed: f32,
    pub r#battlewalkloopspeed: f32,
    pub r#battlerunloopspeed: f32,
    pub r#dashloopspeed: f32,
    pub r#turncw90frame: u8,
    pub r#turnccw90frame: u8,
    pub r#turncw180frame: u8,
    pub r#turnccw180frame: u8,
}
impl CutsceneMotion {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#walkloopspeed: row.field(0usize + offset)?.into_f32()?,
            r#runloopspeed: row.field(1usize + offset)?.into_f32()?,
            r#slowwalkloopspeed: row.field(2usize + offset)?.into_f32()?,
            r#slowrunloopspeed: row.field(3usize + offset)?.into_f32()?,
            r#battlewalkloopspeed: row.field(4usize + offset)?.into_f32()?,
            r#battlerunloopspeed: row.field(5usize + offset)?.into_f32()?,
            r#dashloopspeed: row.field(6usize + offset)?.into_f32()?,
            r#turncw90frame: row.field(7usize + offset)?.into_u8()?,
            r#turnccw90frame: row.field(8usize + offset)?.into_u8()?,
            r#turncw180frame: row.field(9usize + offset)?.into_u8()?,
            r#turnccw180frame: row.field(10usize + offset)?.into_u8()?,
        })
    }
}
