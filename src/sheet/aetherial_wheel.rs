use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for AetherialWheel {
    fn name() -> String {
        "AetherialWheel".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AetherialWheel::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AetherialWheel {
    pub r#item_unprimed: i32,
    pub r#item_primed: i32,
    pub r#grade: u8,
    pub r#hours_required: u8,
}
impl AetherialWheel {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item_unprimed: row.field(0usize + offset)?.into_i32()?,
            r#item_primed: row.field(1usize + offset)?.into_i32()?,
            r#grade: row.field(2usize + offset)?.into_u8()?,
            r#hours_required: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
