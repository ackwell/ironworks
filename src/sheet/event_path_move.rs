use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for EventPathMove {
    fn name() -> String {
        "EventPathMove".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EventPathMove::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EventPathMove {
    pub r#unknown0: u32,
    pub r#unknown1: u16,
    pub r#unknown2: bool,
    pub r#unknown3: u8,
    pub r#unknown4: u16,
    pub r#unknown5: u32,
    pub r#unknown6: u32,
    pub r#unknown7: u32,
    pub r#unknown8: u32,
    pub r#unknown9: u32,
    pub r#unknown10: SeString,
    pub r#unknown11: SeString,
}
impl EventPathMove {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
            r#unknown8: row.field(8usize + offset)?.into_u32()?,
            r#unknown9: row.field(9usize + offset)?.into_u32()?,
            r#unknown10: row.field(10usize + offset)?.into_string()?,
            r#unknown11: row.field(11usize + offset)?.into_string()?,
        })
    }
}
