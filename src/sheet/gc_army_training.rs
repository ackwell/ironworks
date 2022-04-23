use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for GcArmyTraining {
    fn name() -> String {
        "GcArmyTraining".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GcArmyTraining::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GcArmyTraining {
    pub r#physical_bonus: i8,
    pub r#mental_bonus: i8,
    pub r#tactical_bonus: i8,
    pub r#experience: u32,
    pub r#name: SeString,
    pub r#description: SeString,
}
impl GcArmyTraining {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#physical_bonus: row.field(0usize + offset)?.into_i8()?,
            r#mental_bonus: row.field(1usize + offset)?.into_i8()?,
            r#tactical_bonus: row.field(2usize + offset)?.into_i8()?,
            r#experience: row.field(3usize + offset)?.into_u32()?,
            r#name: row.field(4usize + offset)?.into_string()?,
            r#description: row.field(5usize + offset)?.into_string()?,
        })
    }
}
