use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for RecastNavimesh {
    fn name() -> String {
        "RecastNavimesh".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RecastNavimesh::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RecastNavimesh {
    pub r#unknown0: SeString,
    pub r#tile_size: f32,
    pub r#cell_size: f32,
    pub r#cell_height: f32,
    pub r#agent_height: f32,
    pub r#agent_radius: f32,
    pub r#agent_max_climb: f32,
    pub r#agent_max_slope: f32,
    pub r#unknown8: bool,
    pub r#region_min_size: f32,
    pub r#region_merged_size: f32,
    pub r#unknown11: bool,
    pub r#max_edge_length: f32,
    pub r#max_edge_error: f32,
    pub r#verts_per_poly: f32,
    pub r#detail_mesh_sample_distance: f32,
    pub r#detail_mesh_max_sample_error: f32,
}
impl RecastNavimesh {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_string()?,
            r#tile_size: row.field(1usize + offset)?.into_f32()?,
            r#cell_size: row.field(2usize + offset)?.into_f32()?,
            r#cell_height: row.field(3usize + offset)?.into_f32()?,
            r#agent_height: row.field(4usize + offset)?.into_f32()?,
            r#agent_radius: row.field(5usize + offset)?.into_f32()?,
            r#agent_max_climb: row.field(6usize + offset)?.into_f32()?,
            r#agent_max_slope: row.field(7usize + offset)?.into_f32()?,
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
            r#region_min_size: row.field(9usize + offset)?.into_f32()?,
            r#region_merged_size: row.field(10usize + offset)?.into_f32()?,
            r#unknown11: row.field(11usize + offset)?.into_bool()?,
            r#max_edge_length: row.field(12usize + offset)?.into_f32()?,
            r#max_edge_error: row.field(13usize + offset)?.into_f32()?,
            r#verts_per_poly: row.field(14usize + offset)?.into_f32()?,
            r#detail_mesh_sample_distance: row.field(15usize + offset)?.into_f32()?,
            r#detail_mesh_max_sample_error: row.field(16usize + offset)?.into_f32()?,
        })
    }
}
