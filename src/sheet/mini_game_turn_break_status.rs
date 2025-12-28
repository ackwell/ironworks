use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for MiniGameTurnBreakStatus {
    fn name() -> String {
        "MiniGameTurnBreakStatus".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MiniGameTurnBreakStatus::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MiniGameTurnBreakStatus {
    pub r#unknown0: i32,
    pub r#unknown1: u8,
    pub r#unknown2: bool,
    pub r#unknown3: u8,
    pub r#unknown4: SeString,
    pub r#unknown5: SeString,
}
impl MiniGameTurnBreakStatus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_string()?,
            r#unknown5: row.field(5usize + offset)?.into_string()?,
        })
    }
}
