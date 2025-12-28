use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for GCScripShopItem {
    fn name() -> String {
        "GCScripShopItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GCScripShopItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GCScripShopItem {
    pub r#item: i32,
    pub r#required_grand_company_rank: i32,
    pub r#cost_gc_seals: u32,
    pub r#sort_key: u8,
}
impl GCScripShopItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#required_grand_company_rank: row.field(1usize + offset)?.into_i32()?,
            r#cost_gc_seals: row.field(2usize + offset)?.into_u32()?,
            r#sort_key: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
