use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
impl MetadataAdapter for GroupPoseFrame {
    fn name() -> String {
        "GroupPoseFrame".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GroupPoseFrame::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GroupPoseFrame {
    pub r#unknown0: i32,
    pub r#image: i32,
    pub r#grid_text: SeString,
    pub r#unknown3: i32,
    pub r#unknown4: u32,
    pub r#unknown5: u8,
    pub r#unknown6: i32,
    pub r#text: SeString,
}
impl GroupPoseFrame {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i32()?,
            r#image: row.field(1usize + offset)?.into_i32()?,
            r#grid_text: row.field(2usize + offset)?.into_string()?,
            r#unknown3: row.field(3usize + offset)?.into_i32()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_i32()?,
            r#text: row.field(7usize + offset)?.into_string()?,
        })
    }
}
