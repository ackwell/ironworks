use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Treasure {
    fn name() -> String {
        "Treasure".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Treasure::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Treasure {
    pub r#unknown0: SeString,
    pub r#unknown1: i8,
    pub r#unknown2: SeString,
    pub r#unknown3: i8,
    pub r#unknown4: i8,
    pub r#unknown5: i8,
    pub r#unknown6: i8,
    pub r#unknown7: i8,
    pub r#sgb: u32,
    pub r#unknown9: bool,
    pub r#unknown10: bool,
    pub r#unknown11: u8,
}
impl Treasure {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_i8()?,
            r#unknown2: row.field(2usize + offset)?.into_string()?,
            r#unknown3: row.field(3usize + offset)?.into_i8()?,
            r#unknown4: row.field(4usize + offset)?.into_i8()?,
            r#unknown5: row.field(5usize + offset)?.into_i8()?,
            r#unknown6: row.field(6usize + offset)?.into_i8()?,
            r#unknown7: row.field(7usize + offset)?.into_i8()?,
            r#sgb: row.field(8usize + offset)?.into_u32()?,
            r#unknown9: row.field(9usize + offset)?.into_bool()?,
            r#unknown10: row.field(10usize + offset)?.into_bool()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
        })
    }
}
