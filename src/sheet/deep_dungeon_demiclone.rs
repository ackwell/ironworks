use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for DeepDungeonDemiclone {
    fn name() -> String {
        "DeepDungeonDemiclone".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DeepDungeonDemiclone::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DeepDungeonDemiclone {
    pub r#icon: u32,
    pub r#singular: SeString,
    pub r#unknown2: i8,
    pub r#plural: SeString,
    pub r#unknown4: i8,
    pub r#unknown5: i8,
    pub r#unknown6: i8,
    pub r#unknown7: i8,
    pub r#unknown8: i8,
    pub r#title_case: SeString,
    pub r#description: SeString,
}
impl DeepDungeonDemiclone {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_u32()?,
            r#singular: row.field(1usize + offset)?.into_string()?,
            r#unknown2: row.field(2usize + offset)?.into_i8()?,
            r#plural: row.field(3usize + offset)?.into_string()?,
            r#unknown4: row.field(4usize + offset)?.into_i8()?,
            r#unknown5: row.field(5usize + offset)?.into_i8()?,
            r#unknown6: row.field(6usize + offset)?.into_i8()?,
            r#unknown7: row.field(7usize + offset)?.into_i8()?,
            r#unknown8: row.field(8usize + offset)?.into_i8()?,
            r#title_case: row.field(9usize + offset)?.into_string()?,
            r#description: row.field(10usize + offset)?.into_string()?,
        })
    }
}
