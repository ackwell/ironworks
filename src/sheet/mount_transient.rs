use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for MountTransient {
    fn name() -> String {
        "MountTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MountTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MountTransient {
    pub r#description: SeString,
    pub r#description_enhanced: SeString,
    pub r#tooltip: SeString,
}
impl MountTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#description: row.field(0usize + offset)?.into_string()?,
            r#description_enhanced: row.field(1usize + offset)?.into_string()?,
            r#tooltip: row.field(2usize + offset)?.into_string()?,
        })
    }
}
