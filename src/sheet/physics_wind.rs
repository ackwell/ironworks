use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for PhysicsWind {
    fn name() -> String {
        "PhysicsWind".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PhysicsWind::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PhysicsWind {
    pub r#threshold: f32,
    pub r#amplitude: f32,
    pub r#amplitude_frequency: f32,
    pub r#power_min: f32,
    pub r#power_max: f32,
    pub r#power_frequency: f32,
}
impl PhysicsWind {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#threshold: row.field(0usize + offset)?.into_f32()?,
            r#amplitude: row.field(1usize + offset)?.into_f32()?,
            r#amplitude_frequency: row.field(2usize + offset)?.into_f32()?,
            r#power_min: row.field(3usize + offset)?.into_f32()?,
            r#power_max: row.field(4usize + offset)?.into_f32()?,
            r#power_frequency: row.field(5usize + offset)?.into_f32()?,
        })
    }
}
