use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for GroupPoseStamp {
    fn name() -> String {
        "GroupPoseStamp".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GroupPoseStamp::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GroupPoseStamp {
    pub r#stamp_icon: i32,
    pub r#unknown1: i32,
    pub r#category: i32,
    pub r#unknown3: u16,
    pub r#unknown4: u8,
    pub r#unknown5: u32,
    pub r#unknown6: i32,
    pub r#unknown7: bool,
    pub r#unknown8: bool,
    pub r#unknown9: bool,
    pub r#name: SeString,
}
impl GroupPoseStamp {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#stamp_icon: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_i32()?,
            r#category: row.field(2usize + offset)?.into_i32()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
            r#unknown5: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_i32()?,
            r#unknown7: row.field(7usize + offset)?.into_bool()?,
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
            r#unknown9: row.field(9usize + offset)?.into_bool()?,
            r#name: row.field(10usize + offset)?.into_string()?,
        })
    }
}
