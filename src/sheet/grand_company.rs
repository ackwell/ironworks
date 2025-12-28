use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for GrandCompany {
    fn name() -> String {
        "GrandCompany".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GrandCompany::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GrandCompany {
    pub r#name: SeString,
    pub r#unknown1: i8,
    pub r#unknown2: SeString,
    pub r#unknown3: i8,
    pub r#unknown4: i8,
    pub r#unknown5: i8,
    pub r#unknown6: i8,
    pub r#unknown7: i8,
    pub r#unknown8: SeString,
    pub r#unknown9: i8,
}
impl GrandCompany {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_i8()?,
            r#unknown2: row.field(2usize + offset)?.into_string()?,
            r#unknown3: row.field(3usize + offset)?.into_i8()?,
            r#unknown4: row.field(4usize + offset)?.into_i8()?,
            r#unknown5: row.field(5usize + offset)?.into_i8()?,
            r#unknown6: row.field(6usize + offset)?.into_i8()?,
            r#unknown7: row.field(7usize + offset)?.into_i8()?,
            r#unknown8: row.field(8usize + offset)?.into_string()?,
            r#unknown9: row.field(9usize + offset)?.into_i8()?,
        })
    }
}
