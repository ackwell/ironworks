use ironworks::excel::Row;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use crate::utility::read_array;
use std::result::Result;
impl MetadataAdapter for RelicNote {
    fn name() -> String {
        "RelicNote".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RelicNote::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RelicNote_MonsterCon {
    pub r#monster_note_target_common: u16,
    pub r#monster_count: u8,
}
impl RelicNote_MonsterCon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#monster_note_target_common: row.field(1usize + offset)?.into_u16()?,
            r#monster_count: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct RelicNote_Fate {
    pub r#fate: u16,
    pub r#place_name_fate: u16,
}
impl RelicNote_Fate {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#fate: row.field(25usize + offset)?.into_u16()?,
            r#place_name_fate: row.field(26usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct RelicNote {
    pub r#event_item: u32,
    pub r#monster_con: Vec<RelicNote_MonsterCon>,
    pub r#monster_note_target_nm: Vec<u16>,
    pub r#unknown24: u16,
    pub r#fate: Vec<RelicNote_Fate>,
    pub r#leve: Vec<u16>,
}
impl RelicNote {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#event_item: row.field(0usize + offset)?.into_u32()?,
            r#monster_con: read_array(
                offset,
                10usize,
                2usize,
                |offset| { Result::Ok(RelicNote_MonsterCon::populate(row, offset)?) },
            )?,
            r#monster_note_target_nm: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(21usize + offset)?.into_u16()?) },
            )?,
            r#unknown24: row.field(24usize + offset)?.into_u16()?,
            r#fate: read_array(
                offset,
                3usize,
                2usize,
                |offset| { Result::Ok(RelicNote_Fate::populate(row, offset)?) },
            )?,
            r#leve: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(31usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
