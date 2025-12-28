use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Weather {
    fn name() -> String {
        "Weather".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Weather::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Weather {
    pub r#icon: i32,
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#unknown3: SeString,
    pub r#unknown4: SeString,
    pub r#unknown5: SeString,
    pub r#unknown6: SeString,
}
impl Weather {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_i32()?,
            r#name: row.field(1usize + offset)?.into_string()?,
            r#description: row.field(2usize + offset)?.into_string()?,
            r#unknown3: row.field(3usize + offset)?.into_string()?,
            r#unknown4: row.field(4usize + offset)?.into_string()?,
            r#unknown5: row.field(5usize + offset)?.into_string()?,
            r#unknown6: row.field(6usize + offset)?.into_string()?,
        })
    }
}
