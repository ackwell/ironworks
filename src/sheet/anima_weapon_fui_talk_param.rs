use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
impl MetadataAdapter for AnimaWeaponFUITalkParam {
    fn name() -> String {
        "AnimaWeaponFUITalkParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AnimaWeaponFUITalkParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AnimaWeaponFUITalkParam {
    pub r#prologue: SeString,
    pub r#epilogue: SeString,
}
impl AnimaWeaponFUITalkParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#prologue: row.field(0usize + offset)?.into_string()?,
            r#epilogue: row.field(1usize + offset)?.into_string()?,
        })
    }
}
