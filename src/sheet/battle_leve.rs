use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for BattleLeve {
    fn name() -> String {
        "BattleLeve".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BattleLeve::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BattleLeve_ToDoParam {
    pub r#to_do_param: Vec<u32>,
}
impl BattleLeve_ToDoParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#to_do_param: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(64usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
#[derive(Debug)]
pub struct BattleLeve_NumOfAppearance {
    pub r#num_of_appearance: Vec<u8>,
}
impl BattleLeve_NumOfAppearance {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#num_of_appearance: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(104usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
#[derive(Debug)]
pub struct BattleLeve {
    pub r#time: Vec<u16>,
    pub r#base_id: Vec<i32>,
    pub r#enemy_level: Vec<u16>,
    pub r#b_npc_name: Vec<u32>,
    pub r#items_involved: Vec<i32>,
    pub r#items_involved_qty: Vec<u8>,
    pub r#item_drop_rate: Vec<u8>,
    pub r#to_do_number_involved: Vec<u32>,
    pub r#to_do_param: Vec<BattleLeve_ToDoParam>,
    pub r#num_of_appearance: Vec<BattleLeve_NumOfAppearance>,
    pub r#to_do_sequence: Vec<u8>,
    pub r#rule: i32,
    pub r#varient: u8,
    pub r#objective0: u16,
    pub r#objective1: u16,
    pub r#objective2: u16,
    pub r#help0: u16,
    pub r#help1: u16,
}
impl BattleLeve {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#time: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u16()?) },
            )?,
            r#base_id: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(8usize + offset)?.into_i32()?) },
            )?,
            r#enemy_level: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(16usize + offset)?.into_u16()?) },
            )?,
            r#b_npc_name: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(24usize + offset)?.into_u32()?) },
            )?,
            r#items_involved: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(32usize + offset)?.into_i32()?) },
            )?,
            r#items_involved_qty: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(40usize + offset)?.into_u8()?) },
            )?,
            r#item_drop_rate: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(48usize + offset)?.into_u8()?) },
            )?,
            r#to_do_number_involved: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(56usize + offset)?.into_u32()?) },
            )?,
            r#to_do_param: read_array(
                offset,
                5usize,
                8usize,
                |offset| { Result::Ok(BattleLeve_ToDoParam::populate(row, offset)?) },
            )?,
            r#num_of_appearance: read_array(
                offset,
                8usize,
                8usize,
                |offset| {
                    Result::Ok(BattleLeve_NumOfAppearance::populate(row, offset)?)
                },
            )?,
            r#to_do_sequence: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(168usize + offset)?.into_u8()?) },
            )?,
            r#rule: row.field(176usize + offset)?.into_i32()?,
            r#varient: row.field(177usize + offset)?.into_u8()?,
            r#objective0: row.field(178usize + offset)?.into_u16()?,
            r#objective1: row.field(179usize + offset)?.into_u16()?,
            r#objective2: row.field(180usize + offset)?.into_u16()?,
            r#help0: row.field(181usize + offset)?.into_u16()?,
            r#help1: row.field(182usize + offset)?.into_u16()?,
        })
    }
}
