use std::vec::Vec;
use crate::utility::read_array;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MonsterNoteTarget {
    fn name() -> String {
        "MonsterNoteTarget".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MonsterNoteTarget::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MonsterNoteTarget_PlaceNameon {
    pub r#place_name_zone: u16,
    pub r#place_name_location: u16,
}
impl MonsterNoteTarget_PlaceNameon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#place_name_zone: row.field(3usize + offset)?.into_u16()?,
            r#place_name_location: row.field(4usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct MonsterNoteTarget {
    pub r#b_npc_name: u16,
    pub r#icon: i32,
    pub r#town: u8,
    pub r#place_nameon: Vec<MonsterNoteTarget_PlaceNameon>,
}
impl MonsterNoteTarget {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#b_npc_name: row.field(0usize + offset)?.into_u16()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#town: row.field(2usize + offset)?.into_u8()?,
            r#place_nameon: read_array(
                offset,
                3usize,
                2usize,
                |offset| {
                    Result::Ok(MonsterNoteTarget_PlaceNameon::populate(row, offset)?)
                },
            )?,
        })
    }
}
