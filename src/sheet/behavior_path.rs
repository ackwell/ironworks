use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for BehaviorPath {
    fn name() -> String {
        "BehaviorPath".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BehaviorPath::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BehaviorPath {
    pub r#is_turn_transition: bool,
    pub r#is_fade_out: bool,
    pub r#is_fade_in: bool,
    pub r#is_walking: bool,
    pub r#unknown4: bool,
    pub r#speed: f32,
}
impl BehaviorPath {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#is_turn_transition: row.field(0usize + offset)?.into_bool()?,
            r#is_fade_out: row.field(1usize + offset)?.into_bool()?,
            r#is_fade_in: row.field(2usize + offset)?.into_bool()?,
            r#is_walking: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#speed: row.field(5usize + offset)?.into_f32()?,
        })
    }
}
