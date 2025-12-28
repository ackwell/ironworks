use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for SharlayanCraftWorksSupply {
    fn name() -> String {
        "SharlayanCraftWorksSupply".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SharlayanCraftWorksSupply::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SharlayanCraftWorksSupply {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#unknown4: u32,
    pub r#unknown5: u32,
    pub r#unknown6: u32,
    pub r#unknown7: u32,
    pub r#unknown8: u16,
    pub r#unknown9: u16,
    pub r#unknown10: u16,
    pub r#unknown11: u16,
    pub r#unknown12: u16,
    pub r#unknown13: u16,
    pub r#unknown14: u16,
    pub r#unknown15: u16,
    pub r#unknown16: u32,
    pub r#unknown17: u32,
    pub r#unknown18: u32,
    pub r#unknown19: u32,
    pub r#unknown20: u8,
    pub r#unknown21: u8,
    pub r#unknown22: u8,
    pub r#unknown23: u8,
    pub r#unknown24: u16,
    pub r#unknown25: u16,
    pub r#unknown26: u16,
    pub r#unknown27: u16,
    pub r#unknown28: u8,
    pub r#unknown29: u8,
    pub r#unknown30: u8,
    pub r#unknown31: u8,
    pub r#unknown32: u8,
    pub r#unknown33: u8,
    pub r#unknown34: u8,
    pub r#unknown35: u8,
    pub r#unknown36: u8,
    pub r#unknown37: u8,
    pub r#unknown38: u8,
    pub r#unknown39: u8,
    pub r#unknown40: u8,
    pub r#unknown41: u8,
    pub r#unknown42: u8,
    pub r#unknown43: u8,
}
impl SharlayanCraftWorksSupply {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
            r#unknown5: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
            r#unknown8: row.field(8usize + offset)?.into_u16()?,
            r#unknown9: row.field(9usize + offset)?.into_u16()?,
            r#unknown10: row.field(10usize + offset)?.into_u16()?,
            r#unknown11: row.field(11usize + offset)?.into_u16()?,
            r#unknown12: row.field(12usize + offset)?.into_u16()?,
            r#unknown13: row.field(13usize + offset)?.into_u16()?,
            r#unknown14: row.field(14usize + offset)?.into_u16()?,
            r#unknown15: row.field(15usize + offset)?.into_u16()?,
            r#unknown16: row.field(16usize + offset)?.into_u32()?,
            r#unknown17: row.field(17usize + offset)?.into_u32()?,
            r#unknown18: row.field(18usize + offset)?.into_u32()?,
            r#unknown19: row.field(19usize + offset)?.into_u32()?,
            r#unknown20: row.field(20usize + offset)?.into_u8()?,
            r#unknown21: row.field(21usize + offset)?.into_u8()?,
            r#unknown22: row.field(22usize + offset)?.into_u8()?,
            r#unknown23: row.field(23usize + offset)?.into_u8()?,
            r#unknown24: row.field(24usize + offset)?.into_u16()?,
            r#unknown25: row.field(25usize + offset)?.into_u16()?,
            r#unknown26: row.field(26usize + offset)?.into_u16()?,
            r#unknown27: row.field(27usize + offset)?.into_u16()?,
            r#unknown28: row.field(28usize + offset)?.into_u8()?,
            r#unknown29: row.field(29usize + offset)?.into_u8()?,
            r#unknown30: row.field(30usize + offset)?.into_u8()?,
            r#unknown31: row.field(31usize + offset)?.into_u8()?,
            r#unknown32: row.field(32usize + offset)?.into_u8()?,
            r#unknown33: row.field(33usize + offset)?.into_u8()?,
            r#unknown34: row.field(34usize + offset)?.into_u8()?,
            r#unknown35: row.field(35usize + offset)?.into_u8()?,
            r#unknown36: row.field(36usize + offset)?.into_u8()?,
            r#unknown37: row.field(37usize + offset)?.into_u8()?,
            r#unknown38: row.field(38usize + offset)?.into_u8()?,
            r#unknown39: row.field(39usize + offset)?.into_u8()?,
            r#unknown40: row.field(40usize + offset)?.into_u8()?,
            r#unknown41: row.field(41usize + offset)?.into_u8()?,
            r#unknown42: row.field(42usize + offset)?.into_u8()?,
            r#unknown43: row.field(43usize + offset)?.into_u8()?,
        })
    }
}
