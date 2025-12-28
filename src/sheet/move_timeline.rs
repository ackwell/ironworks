use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MoveTimeline {
    fn name() -> String {
        "MoveTimeline".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MoveTimeline::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MoveTimeline {
    pub r#idle: u16,
    pub r#move_forward: u16,
    pub r#move_back: u16,
    pub r#move_left: u16,
    pub r#move_right: u16,
    pub r#move_up: u16,
    pub r#move_down: u16,
    pub r#move_turn_left: u16,
    pub r#move_turn_right: u16,
    pub r#extra: u16,
}
impl MoveTimeline {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#idle: row.field(0usize + offset)?.into_u16()?,
            r#move_forward: row.field(1usize + offset)?.into_u16()?,
            r#move_back: row.field(2usize + offset)?.into_u16()?,
            r#move_left: row.field(3usize + offset)?.into_u16()?,
            r#move_right: row.field(4usize + offset)?.into_u16()?,
            r#move_up: row.field(5usize + offset)?.into_u16()?,
            r#move_down: row.field(6usize + offset)?.into_u16()?,
            r#move_turn_left: row.field(7usize + offset)?.into_u16()?,
            r#move_turn_right: row.field(8usize + offset)?.into_u16()?,
            r#extra: row.field(9usize + offset)?.into_u16()?,
        })
    }
}
