use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for EurekaLogosMixerProbability {
    fn name() -> String {
        "EurekaLogosMixerProbability".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EurekaLogosMixerProbability::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EurekaLogosMixerProbability {
    pub r#probability_percent: u8,
}
impl EurekaLogosMixerProbability {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#probability_percent: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
