use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for FGSAddon {
    fn name() -> String {
        "FGSAddon".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FGSAddon::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FGSAddon {
    pub r#unknown0: SeString,
}
impl FGSAddon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_string()?,
        })
    }
}
