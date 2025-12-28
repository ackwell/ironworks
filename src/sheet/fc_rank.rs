use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for FCRank {
    fn name() -> String {
        "FCRank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FCRank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FCRank {
    pub r#next_point: u32,
    pub r#current_point: u32,
    pub r#rights: u16,
    pub r#unknown3: u16,
    pub r#unknown4: u16,
    pub r#fc_action_active_num: u8,
    pub r#fc_action_stock_num: u8,
    pub r#fc_chest_compartments: u8,
}
impl FCRank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#next_point: row.field(0usize + offset)?.into_u32()?,
            r#current_point: row.field(1usize + offset)?.into_u32()?,
            r#rights: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#fc_action_active_num: row.field(5usize + offset)?.into_u8()?,
            r#fc_action_stock_num: row.field(6usize + offset)?.into_u8()?,
            r#fc_chest_compartments: row.field(7usize + offset)?.into_u8()?,
        })
    }
}
