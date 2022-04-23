use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
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
}
impl GilShop {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_u32()?,
            r#quest: row.field(2usize + offset)?.into_u32()?,
            r#accept_talk: row.field(3usize + offset)?.into_i32()?,
            r#fail_talk: row.field(4usize + offset)?.into_i32()?,
        })
    }
}
