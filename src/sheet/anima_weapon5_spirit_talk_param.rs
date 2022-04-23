use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
impl MetadataAdapter for AnimaWeapon5SpiritTalkParam {
    fn name() -> String {
        "AnimaWeapon5SpiritTalkParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AnimaWeapon5SpiritTalkParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AnimaWeapon5SpiritTalkParam {
    pub r#prologue: SeString,
    pub r#epilogue: SeString,
}
impl AnimaWeapon5SpiritTalkParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#prologue: row.field(0usize + offset)?.into_string()?,
            r#epilogue: row.field(1usize + offset)?.into_string()?,
        })
    }
}
