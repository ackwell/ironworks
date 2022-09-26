use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for CharaCardDesignType {
    fn name() -> String {
        "CharaCardDesignType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CharaCardDesignType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CharaCardDesignType {}
impl CharaCardDesignType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
