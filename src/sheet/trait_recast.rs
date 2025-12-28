use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for TraitRecast {
    fn name() -> String {
        "TraitRecast".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TraitRecast::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TraitRecast {
    pub r#trait: u16,
    pub r#action: u16,
    pub r#timeds: u16,
}
impl TraitRecast {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#trait: row.field(0usize + offset)?.into_u16()?,
            r#action: row.field(1usize + offset)?.into_u16()?,
            r#timeds: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
