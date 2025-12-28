use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ActionIndirection {
    fn name() -> String {
        "ActionIndirection".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActionIndirection::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActionIndirection {
    pub r#name: i32,
    pub r#class_job: i8,
    pub r#previous_combo_action: i32,
}
impl ActionIndirection {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_i32()?,
            r#class_job: row.field(1usize + offset)?.into_i8()?,
            r#previous_combo_action: row.field(2usize + offset)?.into_i32()?,
        })
    }
}
