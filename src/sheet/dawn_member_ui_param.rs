use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for DawnMemberUIParam {
    fn name() -> String {
        "DawnMemberUIParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DawnMemberUIParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DawnMemberUIParam {
    pub r#class_singular: SeString,
    pub r#unknown1: SeString,
    pub r#voice_line: u32,
    pub r#class_plural: SeString,
}
impl DawnMemberUIParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#class_singular: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_string()?,
            r#voice_line: row.field(2usize + offset)?.into_u32()?,
            r#class_plural: row.field(3usize + offset)?.into_string()?,
        })
    }
}
