use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for RelicItem {
    fn name() -> String {
        "RelicItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RelicItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RelicItem {
    pub r#unknown0: u8,
    pub r#gladiator_item: u32,
    pub r#pugilist_item: u32,
    pub r#marauder_item: u32,
    pub r#lancer_item: u32,
    pub r#archer_item: u32,
    pub r#conjurer_item: u32,
    pub r#thaumaturge_item: u32,
    pub r#arcanist_smn_item: u32,
    pub r#arcanist_sch_item: u32,
    pub r#shield_item: u32,
    pub r#rogue_item: u32,
}
impl RelicItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#gladiator_item: row.field(1usize + offset)?.into_u32()?,
            r#pugilist_item: row.field(2usize + offset)?.into_u32()?,
            r#marauder_item: row.field(3usize + offset)?.into_u32()?,
            r#lancer_item: row.field(4usize + offset)?.into_u32()?,
            r#archer_item: row.field(5usize + offset)?.into_u32()?,
            r#conjurer_item: row.field(6usize + offset)?.into_u32()?,
            r#thaumaturge_item: row.field(7usize + offset)?.into_u32()?,
            r#arcanist_smn_item: row.field(8usize + offset)?.into_u32()?,
            r#arcanist_sch_item: row.field(9usize + offset)?.into_u32()?,
            r#shield_item: row.field(10usize + offset)?.into_u32()?,
            r#rogue_item: row.field(11usize + offset)?.into_u32()?,
        })
    }
}
