use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for CollectablesShopItem {
    fn name() -> String {
        "CollectablesShopItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CollectablesShopItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CollectablesShopItem {
    pub r#item: u32,
    pub r#collectables_shop_item_group: u8,
    pub r#level_min: u16,
    pub r#level_max: u16,
    pub r#stars: u8,
    pub r#key: u8,
    pub r#collectables_shop_refine: u16,
    pub r#collectables_shop_reward_scrip: u16,
}
impl CollectablesShopItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u32()?,
            r#collectables_shop_item_group: row.field(1usize + offset)?.into_u8()?,
            r#level_min: row.field(2usize + offset)?.into_u16()?,
            r#level_max: row.field(3usize + offset)?.into_u16()?,
            r#stars: row.field(4usize + offset)?.into_u8()?,
            r#key: row.field(5usize + offset)?.into_u8()?,
            r#collectables_shop_refine: row.field(6usize + offset)?.into_u16()?,
            r#collectables_shop_reward_scrip: row.field(7usize + offset)?.into_u16()?,
        })
    }
}
