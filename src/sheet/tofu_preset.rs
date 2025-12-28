use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for TofuPreset {
    fn name() -> String {
        "TofuPreset".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TofuPreset::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TofuPreset {
    pub r#unknown0: bool,
    pub r#unknown1: i32,
    pub r#unknown2: u16,
    pub r#unknown3: SeString,
    pub r#unknown4: i32,
    pub r#unknown5: i32,
    pub r#unknown6: i32,
    pub r#unknown7: i32,
    pub r#unknown8: i32,
    pub r#unknown9: i32,
    pub r#unknown10: i32,
    pub r#unknown11: i32,
    pub r#unknown12: i32,
    pub r#unknown13: i32,
}
impl TofuPreset {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#unknown1: row.field(1usize + offset)?.into_i32()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_string()?,
            r#unknown4: row.field(4usize + offset)?.into_i32()?,
            r#unknown5: row.field(5usize + offset)?.into_i32()?,
            r#unknown6: row.field(6usize + offset)?.into_i32()?,
            r#unknown7: row.field(7usize + offset)?.into_i32()?,
            r#unknown8: row.field(8usize + offset)?.into_i32()?,
            r#unknown9: row.field(9usize + offset)?.into_i32()?,
            r#unknown10: row.field(10usize + offset)?.into_i32()?,
            r#unknown11: row.field(11usize + offset)?.into_i32()?,
            r#unknown12: row.field(12usize + offset)?.into_i32()?,
            r#unknown13: row.field(13usize + offset)?.into_i32()?,
        })
    }
}
