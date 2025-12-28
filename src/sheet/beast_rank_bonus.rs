use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for BeastRankBonus {
    fn name() -> String {
        "BeastRankBonus".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BeastRankBonus::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BeastRankBonus {
    pub r#neutral: u16,
    pub r#recognized: u16,
    pub r#friendly: u16,
    pub r#trusted: u16,
    pub r#respected: u16,
    pub r#honored: u16,
    pub r#sworn: u16,
    pub r#allied_bloodsworn: u16,
    pub r#item: u32,
    pub r#item_quantity: Vec<u8>,
}
impl BeastRankBonus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#neutral: row.field(0usize + offset)?.into_u16()?,
            r#recognized: row.field(1usize + offset)?.into_u16()?,
            r#friendly: row.field(2usize + offset)?.into_u16()?,
            r#trusted: row.field(3usize + offset)?.into_u16()?,
            r#respected: row.field(4usize + offset)?.into_u16()?,
            r#honored: row.field(5usize + offset)?.into_u16()?,
            r#sworn: row.field(6usize + offset)?.into_u16()?,
            r#allied_bloodsworn: row.field(7usize + offset)?.into_u16()?,
            r#item: row.field(8usize + offset)?.into_u32()?,
            r#item_quantity: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(9usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
