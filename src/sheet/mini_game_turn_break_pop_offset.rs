use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MiniGameTurnBreakPopOffset {
    fn name() -> String {
        "MiniGameTurnBreakPopOffset".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MiniGameTurnBreakPopOffset::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MiniGameTurnBreakPopOffset {
    pub r#unknown0: i8,
    pub r#unknown1: i8,
    pub r#unknown2: i8,
    pub r#unknown3: i8,
    pub r#unknown4: i8,
    pub r#unknown5: i8,
    pub r#unknown6: i8,
    pub r#unknown7: i8,
}
impl MiniGameTurnBreakPopOffset {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i8()?,
            r#unknown1: row.field(1usize + offset)?.into_i8()?,
            r#unknown2: row.field(2usize + offset)?.into_i8()?,
            r#unknown3: row.field(3usize + offset)?.into_i8()?,
            r#unknown4: row.field(4usize + offset)?.into_i8()?,
            r#unknown5: row.field(5usize + offset)?.into_i8()?,
            r#unknown6: row.field(6usize + offset)?.into_i8()?,
            r#unknown7: row.field(7usize + offset)?.into_i8()?,
        })
    }
}
