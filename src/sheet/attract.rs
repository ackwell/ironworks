use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for Attract {
    fn name() -> String {
        "Attract".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Attract::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Attract {
    pub r#max_distance: u16,
    pub r#speed: u16,
    pub r#min_remaining_distance: i16,
    pub r#use_distance_between_hitboxes: bool,
    pub r#direction: u8,
}
impl Attract {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#max_distance: row.field(0usize + offset)?.into_u16()?,
            r#speed: row.field(1usize + offset)?.into_u16()?,
            r#min_remaining_distance: row.field(2usize + offset)?.into_i16()?,
            r#use_distance_between_hitboxes: row.field(3usize + offset)?.into_bool()?,
            r#direction: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
