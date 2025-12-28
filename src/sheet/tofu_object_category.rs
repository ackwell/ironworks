use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for TofuObjectCategory {
    fn name() -> String {
        "TofuObjectCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TofuObjectCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TofuObjectCategory {
    pub r#unknown0: bool,
    pub r#unknown1: u16,
    pub r#unknown2: u8,
    pub r#unknown3: SeString,
}
impl TofuObjectCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_string()?,
        })
    }
}
