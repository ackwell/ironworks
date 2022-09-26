use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for WeaponTimeline {
    fn name() -> String {
        "WeaponTimeline".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WeaponTimeline::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WeaponTimeline {
    pub r#file: SeString,
    pub r#next_weapon_timeline: i16,
}
impl WeaponTimeline {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#file: row.field(0usize + offset)?.into_string()?,
            r#next_weapon_timeline: row.field(1usize + offset)?.into_i16()?,
        })
    }
}
