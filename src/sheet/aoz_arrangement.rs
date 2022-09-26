use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for AOZArrangement {
    fn name() -> String {
        "AOZArrangement".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AOZArrangement::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AOZArrangement {
    pub r#aoz_content_briefing_b_npc: u16,
    pub r#position: u16,
}
impl AOZArrangement {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#aoz_content_briefing_b_npc: row.field(0usize + offset)?.into_u16()?,
            r#position: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
