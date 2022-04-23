use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ContentMemberType {
    fn name() -> String {
        "ContentMemberType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentMemberType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentMemberType {
    pub r#unknown0: bool,
    pub r#unknown1: u8,
    pub r#unknown2: bool,
    pub r#unknown3: bool,
    pub r#unknown4: u8,
    pub r#unknown5: u8,
    pub r#unknown6: u8,
    pub r#unknown7: u8,
    pub r#unknown8: u8,
    pub r#unknown9: u8,
    pub r#tanks_per_party: u8,
    pub r#healers_per_party: u8,
    pub r#melees_per_party: u8,
    pub r#ranged_per_party: u8,
}
impl ContentMemberType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#tanks_per_party: row.field(10usize + offset)?.into_u8()?,
            r#healers_per_party: row.field(11usize + offset)?.into_u8()?,
            r#melees_per_party: row.field(12usize + offset)?.into_u8()?,
            r#ranged_per_party: row.field(13usize + offset)?.into_u8()?,
        })
    }
}
