use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ContentsTutorialPage {
    fn name() -> String {
        "ContentsTutorialPage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentsTutorialPage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentsTutorialPage {
    pub r#image: i32,
    pub r#description: SeString,
}
impl ContentsTutorialPage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#image: row.field(0usize + offset)?.into_i32()?,
            r#description: row.field(1usize + offset)?.into_string()?,
        })
    }
}
