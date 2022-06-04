use bevy::{
	core_pipeline::Opaque3d,
	ecs::system::{
		lifetimeless::{Read, SQuery, SRes},
		SystemParamItem,
	},
	pbr::{
		DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
		SetMeshViewBindGroup,
	},
	prelude::*,
	render::{
		mesh::MeshVertexBufferLayout,
		render_asset::RenderAssets,
		render_phase::{
			DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase, SetItemPipeline,
			TrackedRenderPass,
		},
		render_resource::{
			BindGroupLayout, PipelineCache, RenderPipelineDescriptor, SpecializedMeshPipeline,
			SpecializedMeshPipelineError, SpecializedMeshPipelines,
		},
		renderer::RenderDevice,
		view::ExtractedView,
	},
};

use super::material::{Material, MaterialKey};

pub type RenderMode = Opaque3d;

pub struct Pipeline {
	pub mesh_pipeline: MeshPipeline,
	pub material_layout: BindGroupLayout,
	pub vertex_shader: Handle<Shader>,
	// pub fragment_shader: Handle<Shader>,
	// TODO: is this... a good idea?
	asset_server: AssetServer,
}

impl FromWorld for Pipeline {
	fn from_world(world: &mut World) -> Self {
		let render_device = world.resource::<RenderDevice>();
		let material_layout = Material::bind_group_layout(render_device);

		let mesh_pipeline = world.resource::<MeshPipeline>();
		let asset_server = world.resource::<AssetServer>();

		Pipeline {
			mesh_pipeline: mesh_pipeline.clone(),
			material_layout,

			vertex_shader: asset_server.load("shader/mesh.wgsl"),
			// NOTE: cloning the asset server just clones an internal Arc, so this should be fine... right? Keep an eye on it?
			asset_server: asset_server.clone(),
		}
	}
}

impl SpecializedMeshPipeline for Pipeline {
	// TODO: make a struct for the material side of the key if this grows
	type Key = (MeshPipelineKey, MaterialKey);

	fn specialize(
		&self,
		(pbr_key, material_key): Self::Key,
		layout: &MeshVertexBufferLayout,
	) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
		let mut descriptor = self.mesh_pipeline.specialize(pbr_key, layout)?;

		descriptor.vertex.shader = self.vertex_shader.clone();

		let fragment = descriptor.fragment.as_mut().unwrap();
		fragment.shader = self
			.asset_server
			.load(Material::fragment_shader(material_key));

		descriptor.layout = Some(vec![
			self.mesh_pipeline.view_layout.clone(),
			self.mesh_pipeline.mesh_layout.clone(),
			self.material_layout.clone(),
		]);

		Ok(descriptor)
	}
}

pub type Draw = (
	SetItemPipeline,
	SetMeshViewBindGroup<0>,
	SetMeshBindGroup<1>,
	SetMaterialBindGroup<2>,
	DrawMesh,
);

pub struct SetMaterialBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetMaterialBindGroup<I> {
	type Param = (SRes<RenderAssets<Material>>, SQuery<Read<Handle<Material>>>);

	fn render<'w>(
		_view: Entity,
		item: Entity,
		(materials, query): SystemParamItem<'w, '_, Self::Param>,
		pass: &mut TrackedRenderPass<'w>,
	) -> RenderCommandResult {
		// Get the material on the rendering entity item.
		let material_handle = query.get(item).unwrap();
		let material = materials.into_inner().get(material_handle).unwrap();

		pass.set_bind_group(I, &material.bind_group, &[]);

		RenderCommandResult::Success
	}
}

#[allow(clippy::too_many_arguments)]
pub fn queue(
	draw_functions: Res<DrawFunctions<RenderMode>>,
	render_meshes: Res<RenderAssets<Mesh>>,
	render_materials: Res<RenderAssets<Material>>,
	pipeline: Res<Pipeline>,
	msaa: Res<Msaa>,
	mut pipelines: ResMut<SpecializedMeshPipelines<Pipeline>>,
	mut pipeline_cache: ResMut<PipelineCache>,
	material_meshes: Query<(Entity, &Handle<Mesh>, &MeshUniform, &Handle<Material>)>,
	mut views: Query<(&ExtractedView, &mut RenderPhase<RenderMode>)>,
) {
	let draw = draw_functions.read().get_id::<Draw>().unwrap();
	let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples);
	// TODO: xref this impl with pbr's material, it looks like it's using the view to handle the entities
	for (view, mut phase) in views.iter_mut() {
		let view_matrix = view.transform.compute_matrix();
		let view_row_2 = view_matrix.row(2);

		for (entity, mesh_handle, mesh_uniform, material_handle) in material_meshes.iter() {
			// TODO: just checking for material existence for now - should probably use it to key the pipeline specialisation.
			let material = match render_materials.get(material_handle) {
				Some(material) => material,
				None => continue,
			};

			let mesh = match render_meshes.get(mesh_handle) {
				Some(mesh) => mesh,
				None => continue,
			};

			let pbr_key =
				msaa_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
			let specialized_pipeline = pipelines
				.specialize(
					&mut pipeline_cache,
					&pipeline,
					(pbr_key, Material::key(material)),
					&mesh.layout,
				)
				.unwrap();

			phase.add(RenderMode {
				distance: view_row_2.dot(mesh_uniform.transform.col(3)),
				// fix naming on this so it's thingy
				pipeline: specialized_pipeline,
				entity,
				draw_function: draw,
			})
		}
	}
}
