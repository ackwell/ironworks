use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for MapSymbol {
    fn name() -> String {
        "MapSymbol".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MapSymbol::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MapSymbol {
    pub r#icon: i32,
    pub r#place_name: i32,
    pub r#display_navi: bool,
}
impl MapSymbol {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_i32()?,
            r#place_name: row.field(1usize + offset)?.into_i32()?,
            r#display_navi: row.field(2usize + offset)?.into_bool()?,
        })
    }
}
