use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MJIDisposalShopItem {
    fn name() -> String {
        "MJIDisposalShopItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIDisposalShopItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIDisposalShopItem {
    pub r#item: u8,
    pub r#currency: u8,
    pub r#count: u16,
    pub r#category: u8,
    pub r#sort: u8,
}
impl MJIDisposalShopItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u8()?,
            r#currency: row.field(1usize + offset)?.into_u8()?,
            r#count: row.field(2usize + offset)?.into_u16()?,
            r#category: row.field(3usize + offset)?.into_u8()?,
            r#sort: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
