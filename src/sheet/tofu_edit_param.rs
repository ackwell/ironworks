use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for TofuEditParam {
    fn name() -> String {
        "TofuEditParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TofuEditParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TofuEditParam {
    pub r#unknown0: SeString,
    pub r#unknown1: i16,
    pub r#unknown2: i16,
    pub r#unknown3: i16,
    pub r#unknown4: bool,
}
impl TofuEditParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_i16()?,
            r#unknown2: row.field(2usize + offset)?.into_i16()?,
            r#unknown3: row.field(3usize + offset)?.into_i16()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
        })
    }
}
