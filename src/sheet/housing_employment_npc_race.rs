use ironworks::sestring::SeString;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for HousingEmploymentNpcRace {
    fn name() -> String {
        "HousingEmploymentNpcRace".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HousingEmploymentNpcRace::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HousingEmploymentNpcRace {
    pub r#race: SeString,
}
impl HousingEmploymentNpcRace {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#race: row.field(0usize + offset)?.into_string()?,
        })
    }
}
