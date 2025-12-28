use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Omen {
    fn name() -> String {
        "Omen".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Omen::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Omen {
    pub r#path: SeString,
    pub r#path_ally: SeString,
    pub r#type: u8,
    pub r#restrict_y_scale: bool,
    pub r#large_scale: bool,
    pub r#unknown5: i8,
}
impl Omen {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#path: row.field(0usize + offset)?.into_string()?,
            r#path_ally: row.field(1usize + offset)?.into_string()?,
            r#type: row.field(2usize + offset)?.into_u8()?,
            r#restrict_y_scale: row.field(3usize + offset)?.into_bool()?,
            r#large_scale: row.field(4usize + offset)?.into_bool()?,
            r#unknown5: row.field(5usize + offset)?.into_i8()?,
        })
    }
}
