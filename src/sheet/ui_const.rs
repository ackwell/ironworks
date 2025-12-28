use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for UIConst {
    fn name() -> String {
        "UIConst".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(UIConst::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct UIConst {
    pub r#unknown0: i32,
}
impl UIConst {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
