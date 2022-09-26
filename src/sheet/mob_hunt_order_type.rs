use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MobHuntOrderType {
    fn name() -> String {
        "MobHuntOrderType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MobHuntOrderType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MobHuntOrderType {
    pub r#type: u8,
    pub r#quest: u32,
    pub r#event_item: u32,
    pub r#order_start: u16,
    pub r#order_amount: u8,
}
impl MobHuntOrderType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#quest: row.field(1usize + offset)?.into_u32()?,
            r#event_item: row.field(2usize + offset)?.into_u32()?,
            r#order_start: row.field(3usize + offset)?.into_u16()?,
            r#order_amount: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
