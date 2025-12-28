use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Channeling {
    fn name() -> String {
        "Channeling".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Channeling::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Channeling {
    pub r#file: SeString,
    pub r#width_scale: u8,
    pub r#unknown2: bool,
    pub r#unknown3: bool,
    pub r#unknown4: bool,
    pub r#unknown5: bool,
}
impl Channeling {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#file: row.field(0usize + offset)?.into_string()?,
            r#width_scale: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#unknown5: row.field(5usize + offset)?.into_bool()?,
        })
    }
}
