use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use crate::utility::read_array;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for FccShop {
    fn name() -> String {
        "FccShop".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FccShop::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FccShop_Item {
    pub r#item: u32,
}
impl FccShop_Item {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
#[derive(Debug)]
pub struct FccShop_Cost {
    pub r#cost: u32,
}
impl FccShop_Cost {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#cost: row.field(11usize + offset)?.into_u32()?,
        })
    }
}
#[derive(Debug)]
pub struct FccShop_FCRankRequired {
    pub r#fc_rank_required: u8,
}
impl FccShop_FCRankRequired {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#fc_rank_required: row.field(21usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct FccShop {
    pub r#name: SeString,
    pub r#item: Vec<FccShop_Item>,
    pub r#cost: Vec<FccShop_Cost>,
    pub r#fc_rank_required: Vec<FccShop_FCRankRequired>,
}
impl FccShop {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#item: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(FccShop_Item::populate(row, offset)?) },
            )?,
            r#cost: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(FccShop_Cost::populate(row, offset)?) },
            )?,
            r#fc_rank_required: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(FccShop_FCRankRequired::populate(row, offset)?) },
            )?,
        })
    }
}
