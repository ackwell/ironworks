use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for EquipRaceCategory {
    fn name() -> String {
        "EquipRaceCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EquipRaceCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EquipRaceCategory {
    pub r#hyur: bool,
    pub r#elezen: bool,
    pub r#lalafell: bool,
    pub r#miqote: bool,
    pub r#roegadyn: bool,
    pub r#au_ra: bool,
    pub r#unknown6: bool,
    pub r#unknown7: bool,
    pub r#male: bool,
    pub r#female: bool,
}
impl EquipRaceCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#hyur: row.field(0usize + offset)?.into_bool()?,
            r#elezen: row.field(1usize + offset)?.into_bool()?,
            r#lalafell: row.field(2usize + offset)?.into_bool()?,
            r#miqote: row.field(3usize + offset)?.into_bool()?,
            r#roegadyn: row.field(4usize + offset)?.into_bool()?,
            r#au_ra: row.field(5usize + offset)?.into_bool()?,
            r#unknown6: row.field(6usize + offset)?.into_bool()?,
            r#unknown7: row.field(7usize + offset)?.into_bool()?,
            r#male: row.field(8usize + offset)?.into_bool()?,
            r#female: row.field(9usize + offset)?.into_bool()?,
        })
    }
}
