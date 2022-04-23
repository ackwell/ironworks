use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for GrandCompanyRank {
    fn name() -> String {
        "GrandCompanyRank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GrandCompanyRank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GrandCompanyRank {
    pub r#tier: u8,
    pub r#order: u8,
    pub r#max_seals: u32,
    pub r#required_seals: u32,
    pub r#icon_maelstrom: i32,
    pub r#icon_serpents: i32,
    pub r#icon_flames: i32,
    pub r#quest_maelstrom: i32,
    pub r#quest_serpents: i32,
    pub r#quest_flames: i32,
}
impl GrandCompanyRank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#tier: row.field(0usize + offset)?.into_u8()?,
            r#order: row.field(1usize + offset)?.into_u8()?,
            r#max_seals: row.field(2usize + offset)?.into_u32()?,
            r#required_seals: row.field(3usize + offset)?.into_u32()?,
            r#icon_maelstrom: row.field(4usize + offset)?.into_i32()?,
            r#icon_serpents: row.field(5usize + offset)?.into_i32()?,
            r#icon_flames: row.field(6usize + offset)?.into_i32()?,
            r#quest_maelstrom: row.field(7usize + offset)?.into_i32()?,
            r#quest_serpents: row.field(8usize + offset)?.into_i32()?,
            r#quest_flames: row.field(9usize + offset)?.into_i32()?,
        })
    }
}
