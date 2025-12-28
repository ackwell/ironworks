use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for DisposalShopItem {
    fn name() -> String {
        "DisposalShopItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DisposalShopItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DisposalShopItem {
    pub r#item_disposed: i32,
    pub r#unknown1: bool,
    pub r#item_received: i32,
    pub r#unknown3: bool,
    pub r#quantity_received: u32,
    pub r#unknown5: u16,
}
impl DisposalShopItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item_disposed: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#item_received: row.field(2usize + offset)?.into_i32()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#quantity_received: row.field(4usize + offset)?.into_u32()?,
            r#unknown5: row.field(5usize + offset)?.into_u16()?,
        })
    }
}
