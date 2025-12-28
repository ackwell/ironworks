use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for FittingShopItemSet {
    fn name() -> String {
        "FittingShopItemSet".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FittingShopItemSet::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FittingShopItemSet {
    pub r#unknown0: i32,
    pub r#unknown1: i32,
    pub r#unknown2: i32,
    pub r#unknown3: i32,
    pub r#unknown4: i32,
    pub r#unknown5: i32,
    pub r#unknown6: SeString,
}
impl FittingShopItemSet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_i32()?,
            r#unknown2: row.field(2usize + offset)?.into_i32()?,
            r#unknown3: row.field(3usize + offset)?.into_i32()?,
            r#unknown4: row.field(4usize + offset)?.into_i32()?,
            r#unknown5: row.field(5usize + offset)?.into_i32()?,
            r#unknown6: row.field(6usize + offset)?.into_string()?,
        })
    }
}
