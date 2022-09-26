use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for BuddySkill {
    fn name() -> String {
        "BuddySkill".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BuddySkill::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BuddySkill {
    pub r#buddy_level: u8,
    pub r#is_active: bool,
    pub r#defender: u16,
    pub r#attacker: u16,
    pub r#healer: u16,
}
impl BuddySkill {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#buddy_level: row.field(0usize + offset)?.into_u8()?,
            r#is_active: row.field(1usize + offset)?.into_bool()?,
            r#defender: row.field(2usize + offset)?.into_u16()?,
            r#attacker: row.field(3usize + offset)?.into_u16()?,
            r#healer: row.field(4usize + offset)?.into_u16()?,
        })
    }
}
