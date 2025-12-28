use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for BGMSystemDefine {
    fn name() -> String {
        "BGMSystemDefine".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BGMSystemDefine::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BGMSystemDefine {
    pub r#define: f32,
}
impl BGMSystemDefine {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#define: row.field(0usize + offset)?.into_f32()?,
        })
    }
}
