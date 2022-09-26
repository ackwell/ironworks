use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for HousingYardObject {
    fn name() -> String {
        "HousingYardObject".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HousingYardObject::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HousingYardObject {
    pub r#model_key: u16,
    pub r#housing_item_category: u8,
    pub r#usage_type: u8,
    pub r#usage_parameter: u32,
    pub r#unknown4: u8,
    pub r#custom_talk: u32,
    pub r#item: u32,
    pub r#destroy_on_removal: bool,
}
impl HousingYardObject {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#model_key: row.field(0usize + offset)?.into_u16()?,
            r#housing_item_category: row.field(1usize + offset)?.into_u8()?,
            r#usage_type: row.field(2usize + offset)?.into_u8()?,
            r#usage_parameter: row.field(3usize + offset)?.into_u32()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
            r#custom_talk: row.field(5usize + offset)?.into_u32()?,
            r#item: row.field(6usize + offset)?.into_u32()?,
            r#destroy_on_removal: row.field(7usize + offset)?.into_bool()?,
        })
    }
}
