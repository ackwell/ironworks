use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for GcArmyExpeditionMemberBonus {
    fn name() -> String {
        "GcArmyExpeditionMemberBonus".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GcArmyExpeditionMemberBonus::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GcArmyExpeditionMemberBonus {
    pub r#race: u8,
    pub r#class_job: u8,
}
impl GcArmyExpeditionMemberBonus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#race: row.field(0usize + offset)?.into_u8()?,
            r#class_job: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
