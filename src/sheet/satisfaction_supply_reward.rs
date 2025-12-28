use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for SatisfactionSupplyReward {
    fn name() -> String {
        "SatisfactionSupplyReward".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SatisfactionSupplyReward::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SatisfactionSupplyReward_uny {
    pub r#reward_currency: u16,
    pub r#quantity_low: u16,
    pub r#quantity_mid: u16,
    pub r#quantity_high: u16,
}
impl SatisfactionSupplyReward_uny {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#reward_currency: row.field(1usize + offset)?.into_u16()?,
            r#quantity_low: row.field(2usize + offset)?.into_u16()?,
            r#quantity_mid: row.field(3usize + offset)?.into_u16()?,
            r#quantity_high: row.field(4usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct SatisfactionSupplyReward {
    pub r#unknown0: u8,
    pub r#uny: Vec<SatisfactionSupplyReward_uny>,
    pub r#unknown9: u8,
    pub r#satisfaction_low: u16,
    pub r#satisfaction_mid: u16,
    pub r#satisfaction_high: u16,
    pub r#gil_low: u16,
    pub r#gil_mid: u16,
    pub r#gil_high: u16,
}
impl SatisfactionSupplyReward {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#uny: read_array(
                offset,
                2usize,
                4usize,
                |offset| {
                    Result::Ok(SatisfactionSupplyReward_uny::populate(row, offset)?)
                },
            )?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#satisfaction_low: row.field(10usize + offset)?.into_u16()?,
            r#satisfaction_mid: row.field(11usize + offset)?.into_u16()?,
            r#satisfaction_high: row.field(12usize + offset)?.into_u16()?,
            r#gil_low: row.field(13usize + offset)?.into_u16()?,
            r#gil_mid: row.field(14usize + offset)?.into_u16()?,
            r#gil_high: row.field(15usize + offset)?.into_u16()?,
        })
    }
}
