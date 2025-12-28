use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for NotoriousMonsterTerritory {
    fn name() -> String {
        "NotoriousMonsterTerritory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(NotoriousMonsterTerritory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct NotoriousMonsterTerritory {
    pub r#notorious_monsters: Vec<u16>,
}
impl NotoriousMonsterTerritory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#notorious_monsters: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
