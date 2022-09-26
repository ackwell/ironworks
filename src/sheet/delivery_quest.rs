use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for DeliveryQuest {
    fn name() -> String {
        "DeliveryQuest".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DeliveryQuest::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DeliveryQuest {
    pub r#quest: i32,
}
impl DeliveryQuest {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
