use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for MonsterNote {
    fn name() -> String {
        "MonsterNote".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MonsterNote::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MonsterNote {
    pub r#monster_note_target: Vec<u16>,
    pub r#count: Vec<u8>,
    pub r#reward: u32,
    pub r#name: SeString,
}
impl MonsterNote {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#monster_note_target: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u16()?) },
            )?,
            r#count: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(4usize + offset)?.into_u8()?) },
            )?,
            r#reward: row.field(8usize + offset)?.into_u32()?,
            r#name: row.field(9usize + offset)?.into_string()?,
        })
    }
}
