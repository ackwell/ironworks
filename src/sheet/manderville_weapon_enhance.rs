use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MandervilleWeaponEnhance {
    fn name() -> String {
        "MandervilleWeaponEnhance".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MandervilleWeaponEnhance::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MandervilleWeaponEnhance {
    pub r#unknown0: u8,
    pub r#unknown1: u32,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#unknown4: u32,
    pub r#unknown5: u8,
    pub r#unknown6: u8,
    pub r#unknown7: u32,
    pub r#unknown8: u8,
    pub r#unknown9: u8,
    pub r#unknown10: u32,
    pub r#unknown11: u8,
    pub r#unknown12: u8,
    pub r#unknown13: u32,
    pub r#unknown14: u16,
    pub r#unknown15: u32,
}
impl MandervilleWeaponEnhance {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#unknown9: row.field(9usize + offset)?.into_u8()?,
            r#unknown10: row.field(10usize + offset)?.into_u32()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
            r#unknown13: row.field(13usize + offset)?.into_u32()?,
            r#unknown14: row.field(14usize + offset)?.into_u16()?,
            r#unknown15: row.field(15usize + offset)?.into_u32()?,
        })
    }
}
