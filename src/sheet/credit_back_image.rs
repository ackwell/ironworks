use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for CreditBackImage {
    fn name() -> String {
        "CreditBackImage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CreditBackImage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CreditBackImage {
    pub r#unknown0: u16,
    pub r#unknown1: u16,
    pub r#unknown2: bool,
    pub r#unknown3: bool,
    pub r#unknown4: bool,
    pub r#back_image: u32,
    pub r#unknown6: u8,
}
impl CreditBackImage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#back_image: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
        })
    }
}
