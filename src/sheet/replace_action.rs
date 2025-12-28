use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ReplaceAction {
    fn name() -> String {
        "ReplaceAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ReplaceAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ReplaceAction {
    pub r#action: i32,
    pub r#type1: i8,
    pub r#param1: i16,
    pub r#replace_action1: i32,
    pub r#type2: i8,
    pub r#param2: i16,
    pub r#replace_action2: i32,
    pub r#type3: i8,
    pub r#param3: i16,
    pub r#replace_action3: i32,
    pub r#replace_settable: i8,
    pub r#unknown11: i16,
    pub r#unknown12: i32,
    pub r#unknown13: i8,
    pub r#unknown14: bool,
}
impl ReplaceAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#action: row.field(0usize + offset)?.into_i32()?,
            r#type1: row.field(1usize + offset)?.into_i8()?,
            r#param1: row.field(2usize + offset)?.into_i16()?,
            r#replace_action1: row.field(3usize + offset)?.into_i32()?,
            r#type2: row.field(4usize + offset)?.into_i8()?,
            r#param2: row.field(5usize + offset)?.into_i16()?,
            r#replace_action2: row.field(6usize + offset)?.into_i32()?,
            r#type3: row.field(7usize + offset)?.into_i8()?,
            r#param3: row.field(8usize + offset)?.into_i16()?,
            r#replace_action3: row.field(9usize + offset)?.into_i32()?,
            r#replace_settable: row.field(10usize + offset)?.into_i8()?,
            r#unknown11: row.field(11usize + offset)?.into_i16()?,
            r#unknown12: row.field(12usize + offset)?.into_i32()?,
            r#unknown13: row.field(13usize + offset)?.into_i8()?,
            r#unknown14: row.field(14usize + offset)?.into_bool()?,
        })
    }
}
