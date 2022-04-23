use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
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
