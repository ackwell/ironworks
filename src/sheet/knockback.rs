use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for Knockback {
    fn name() -> String {
        "Knockback".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Knockback::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Knockback {
    pub r#distance: u8,
    pub r#speed: u8,
    pub r#motion: bool,
    pub r#near_distance: u8,
    pub r#direction: u8,
    pub r#direction_arg: u8,
    pub r#cancel_move: bool,
}
impl Knockback {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#distance: row.field(0usize + offset)?.into_u8()?,
            r#speed: row.field(1usize + offset)?.into_u8()?,
            r#motion: row.field(2usize + offset)?.into_bool()?,
            r#near_distance: row.field(3usize + offset)?.into_u8()?,
            r#direction: row.field(4usize + offset)?.into_u8()?,
            r#direction_arg: row.field(5usize + offset)?.into_u8()?,
            r#cancel_move: row.field(6usize + offset)?.into_bool()?,
        })
    }
}
