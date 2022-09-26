use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for AOZContentBriefingBNpc {
    fn name() -> String {
        "AOZContentBriefingBNpc".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AOZContentBriefingBNpc::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AOZContentBriefingBNpc {
    pub r#b_npc_name: u32,
    pub r#target_small: u32,
    pub r#target_large: u32,
    pub r#hide_stats: bool,
    pub r#endurance: u8,
    pub r#fire: u8,
    pub r#ice: u8,
    pub r#wind: u8,
    pub r#earth: u8,
    pub r#thunder: u8,
    pub r#water: u8,
    pub r#slashing: u8,
    pub r#piercing: u8,
    pub r#blunt: u8,
    pub r#magic: u8,
    pub r#slow_vuln: bool,
    pub r#petrification_vuln: bool,
    pub r#paralysis_vuln: bool,
    pub r#interruption_vuln: bool,
    pub r#blind_vuln: bool,
    pub r#stun_vuln: bool,
    pub r#sleep_vuln: bool,
    pub r#bind_vuln: bool,
    pub r#heavy_vuln: bool,
    pub r#flat_or_death_vuln: bool,
}
impl AOZContentBriefingBNpc {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#b_npc_name: row.field(0usize + offset)?.into_u32()?,
            r#target_small: row.field(1usize + offset)?.into_u32()?,
            r#target_large: row.field(2usize + offset)?.into_u32()?,
            r#hide_stats: row.field(3usize + offset)?.into_bool()?,
            r#endurance: row.field(4usize + offset)?.into_u8()?,
            r#fire: row.field(5usize + offset)?.into_u8()?,
            r#ice: row.field(6usize + offset)?.into_u8()?,
            r#wind: row.field(7usize + offset)?.into_u8()?,
            r#earth: row.field(8usize + offset)?.into_u8()?,
            r#thunder: row.field(9usize + offset)?.into_u8()?,
            r#water: row.field(10usize + offset)?.into_u8()?,
            r#slashing: row.field(11usize + offset)?.into_u8()?,
            r#piercing: row.field(12usize + offset)?.into_u8()?,
            r#blunt: row.field(13usize + offset)?.into_u8()?,
            r#magic: row.field(14usize + offset)?.into_u8()?,
            r#slow_vuln: row.field(15usize + offset)?.into_bool()?,
            r#petrification_vuln: row.field(16usize + offset)?.into_bool()?,
            r#paralysis_vuln: row.field(17usize + offset)?.into_bool()?,
            r#interruption_vuln: row.field(18usize + offset)?.into_bool()?,
            r#blind_vuln: row.field(19usize + offset)?.into_bool()?,
            r#stun_vuln: row.field(20usize + offset)?.into_bool()?,
            r#sleep_vuln: row.field(21usize + offset)?.into_bool()?,
            r#bind_vuln: row.field(22usize + offset)?.into_bool()?,
            r#heavy_vuln: row.field(23usize + offset)?.into_bool()?,
            r#flat_or_death_vuln: row.field(24usize + offset)?.into_bool()?,
        })
    }
}
