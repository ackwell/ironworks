use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for GilShop {
    fn name() -> String {
        "GilShop".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GilShop::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GilShop {
    pub r#name: SeString,
    pub r#icon: u32,
    pub r#quest: u32,
    pub r#accept_talk: i32,
    pub r#fail_talk: i32,
    pub r#unknown5: bool,
    pub r#unknown6: u16,
    pub r#unknown7: u16,
}
impl GilShop {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_u32()?,
            r#quest: row.field(2usize + offset)?.into_u32()?,
            r#accept_talk: row.field(3usize + offset)?.into_i32()?,
            r#fail_talk: row.field(4usize + offset)?.into_i32()?,
            r#unknown5: row.field(5usize + offset)?.into_bool()?,
            r#unknown6: row.field(6usize + offset)?.into_u16()?,
            r#unknown7: row.field(7usize + offset)?.into_u16()?,
        })
    }
}
