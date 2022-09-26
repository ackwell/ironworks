use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
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
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#unknown2: u16,
    pub r#category: u8,
}
impl MJIDisposalShopItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#category: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
