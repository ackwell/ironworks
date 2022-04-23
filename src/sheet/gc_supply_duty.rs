use std::result::Result;
use std::vec::Vec;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for GCSupplyDuty {
    fn name() -> String {
        "GCSupplyDuty".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GCSupplyDuty::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GCSupplyDuty_Item {
    pub r#item: i32,
    pub r#item_count: u8,
}
impl GCSupplyDuty_Item {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#item_count: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct GCSupplyDuty {
    pub r#item: Vec<Vec<GCSupplyDuty_Item>>,
}
impl GCSupplyDuty {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: read_array(
                offset,
                11usize,
                6usize,
                |offset| {
                    Result::Ok(
                        read_array(
                            offset,
                            3usize,
                            2usize,
                            |offset| {
                                Result::Ok(GCSupplyDuty_Item::populate(row, offset)?)
                            },
                        )?,
                    )
                },
            )?,
        })
    }
}
