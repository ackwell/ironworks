use std::result::Result;
use std::vec::Vec;
use crate::utility::read_array;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for GcArmyExpedition {
    fn name() -> String {
        "GcArmyExpedition".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GcArmyExpedition::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GcArmyExpedition_RewardItem {
    pub r#reward_item: i32,
}
impl GcArmyExpedition_RewardItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#reward_item: row.field(10usize + offset)?.into_i32()?,
        })
    }
}
#[derive(Debug)]
pub struct GcArmyExpedition_RewardQuantity {
    pub r#reward_quantity: u8,
}
impl GcArmyExpedition_RewardQuantity {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#reward_quantity: row.field(16usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct GcArmyExpedition_RequiredPhysical {
    pub r#required_physical: u16,
}
impl GcArmyExpedition_RequiredPhysical {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#required_physical: row.field(22usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct GcArmyExpedition_PercentPhysicalMet {
    pub r#percent_physical_met: u8,
}
impl GcArmyExpedition_PercentPhysicalMet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#percent_physical_met: row.field(28usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct GcArmyExpedition_RequiredMental {
    pub r#required_mental: u16,
}
impl GcArmyExpedition_RequiredMental {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#required_mental: row.field(34usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct GcArmyExpedition_PercentMentalMet {
    pub r#percent_mental_met: u8,
}
impl GcArmyExpedition_PercentMentalMet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#percent_mental_met: row.field(40usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct GcArmyExpedition_RequiredTactical {
    pub r#required_tactical: u16,
}
impl GcArmyExpedition_RequiredTactical {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#required_tactical: row.field(46usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct GcArmyExpedition_PercentTacticalMet {
    pub r#percent_tactical_met: u8,
}
impl GcArmyExpedition_PercentTacticalMet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#percent_tactical_met: row.field(52usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct GcArmyExpedition_PercentAllMet {
    pub r#percent_all_met: u8,
}
impl GcArmyExpedition_PercentAllMet {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#percent_all_met: row.field(58usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct GcArmyExpedition {
    pub r#required_flag: u8,
    pub r#unlock_flag: u8,
    pub r#required_level: u8,
    pub r#required_seals: u16,
    pub r#reward_experience: u32,
    pub r#percent_base: u8,
    pub r#unknown6: u8,
    pub r#gc_army_expedition_type: u8,
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#reward_item: Vec<GcArmyExpedition_RewardItem>,
    pub r#reward_quantity: Vec<GcArmyExpedition_RewardQuantity>,
    pub r#required_physical: Vec<GcArmyExpedition_RequiredPhysical>,
    pub r#percent_physical_met: Vec<GcArmyExpedition_PercentPhysicalMet>,
    pub r#required_mental: Vec<GcArmyExpedition_RequiredMental>,
    pub r#percent_mental_met: Vec<GcArmyExpedition_PercentMentalMet>,
    pub r#required_tactical: Vec<GcArmyExpedition_RequiredTactical>,
    pub r#percent_tactical_met: Vec<GcArmyExpedition_PercentTacticalMet>,
    pub r#percent_all_met: Vec<GcArmyExpedition_PercentAllMet>,
}
impl GcArmyExpedition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#required_flag: row.field(0usize + offset)?.into_u8()?,
            r#unlock_flag: row.field(1usize + offset)?.into_u8()?,
            r#required_level: row.field(2usize + offset)?.into_u8()?,
            r#required_seals: row.field(3usize + offset)?.into_u16()?,
            r#reward_experience: row.field(4usize + offset)?.into_u32()?,
            r#percent_base: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#gc_army_expedition_type: row.field(7usize + offset)?.into_u8()?,
            r#name: row.field(8usize + offset)?.into_string()?,
            r#description: row.field(9usize + offset)?.into_string()?,
            r#reward_item: read_array(
                offset,
                6usize,
                1usize,
                |offset| {
                    Result::Ok(GcArmyExpedition_RewardItem::populate(row, offset)?)
                },
            )?,
            r#reward_quantity: read_array(
                offset,
                6usize,
                1usize,
                |offset| {
                    Result::Ok(GcArmyExpedition_RewardQuantity::populate(row, offset)?)
                },
            )?,
            r#required_physical: read_array(
                offset,
                6usize,
                1usize,
                |offset| {
                    Result::Ok(GcArmyExpedition_RequiredPhysical::populate(row, offset)?)
                },
            )?,
            r#percent_physical_met: read_array(
                offset,
                6usize,
                1usize,
                |offset| {
                    Result::Ok(
                        GcArmyExpedition_PercentPhysicalMet::populate(row, offset)?,
                    )
                },
            )?,
            r#required_mental: read_array(
                offset,
                6usize,
                1usize,
                |offset| {
                    Result::Ok(GcArmyExpedition_RequiredMental::populate(row, offset)?)
                },
            )?,
            r#percent_mental_met: read_array(
                offset,
                6usize,
                1usize,
                |offset| {
                    Result::Ok(GcArmyExpedition_PercentMentalMet::populate(row, offset)?)
                },
            )?,
            r#required_tactical: read_array(
                offset,
                6usize,
                1usize,
                |offset| {
                    Result::Ok(GcArmyExpedition_RequiredTactical::populate(row, offset)?)
                },
            )?,
            r#percent_tactical_met: read_array(
                offset,
                6usize,
                1usize,
                |offset| {
                    Result::Ok(
                        GcArmyExpedition_PercentTacticalMet::populate(row, offset)?,
                    )
                },
            )?,
            r#percent_all_met: read_array(
                offset,
                6usize,
                1usize,
                |offset| {
                    Result::Ok(GcArmyExpedition_PercentAllMet::populate(row, offset)?)
                },
            )?,
        })
    }
}
