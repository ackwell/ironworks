use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use crate::error::PopulateError;
use crate::utility::read_array;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for WeeklyLotBonus {
    fn name() -> String {
        "WeeklyLotBonus".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WeeklyLotBonus::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WeeklyLotBonus {
    pub r#weekly_lot_bonus_threshold: Vec<u8>,
}
impl WeeklyLotBonus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#weekly_lot_bonus_threshold: read_array(
                offset,
                31usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
