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
	reflect::TypeUuid,
	render::{
		mesh::MeshVertexBufferLayout,
		render_asset::{PrepareAssetError, RenderAsset, RenderAssetPlugin, RenderAssets},
		render_component::ExtractComponentPlugin,
		render_phase::{
			AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
			SetItemPipeline, TrackedRenderPass,
		},
		render_resource::{
			BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
			BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
			PipelineCache, RenderPipelineDescriptor, SamplerBindingType, ShaderStages,
			SpecializedMeshPipeline, SpecializedMeshPipelineError, SpecializedMeshPipelines,
			TextureSampleType, TextureViewDimension,
		},
		renderer::RenderDevice,
		view::ExtractedView,
		RenderApp, RenderStage,
	},
};

// TODO: should this be opaque in the long run?
type RenderMode = Opaque3d;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
	fn build(&self, app: &mut App) {
		app.add_asset::<Material>()
			.add_plugin(ExtractComponentPlugin::<Handle<Material>>::default())
			.add_plugin(RenderAssetPlugin::<Material>::default());

		app.sub_app_mut(RenderApp)
			.add_render_command::<RenderMode, Draw>()
			.init_resource::<Pipeline>()
			.init_resource::<SpecializedMeshPipelines<Pipeline>>()
			.add_system_to_stage(RenderStage::Queue, queue);
	}
}

#[derive(Clone, TypeUuid)]
#[uuid = "317a2fbb-6fb4-4bbd-b480-1d5942345cc0"]
pub struct Material {
	// TODO: the rest. if ending up with shaders from the game files, this will need revisiting.
	pub color_map_0: Option<Handle<Image>>,
}

pub struct GpuMaterial {
	bind_group: BindGroup,
}

impl RenderAsset for Material {
	type ExtractedAsset = Self;
	type PreparedAsset = GpuMaterial;
	type Param = (
		SRes<RenderDevice>,
		SRes<Pipeline>,
		SRes<RenderAssets<Image>>,
	);

	fn extract_asset(&self) -> Self::ExtractedAsset {
		self.clone()
	}

	fn prepare_asset(
		extracted_asset: Self::ExtractedAsset,
		(render_device, pipeline, images): &mut SystemParamItem<Self::Param>,
	) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
		// TODO: Dedupe this pattern
		let (color_map_0_view, color_map_0_sampler) = match pipeline
			.mesh_pipeline
			.get_image_texture(images, &extracted_asset.color_map_0)
		{
			Some(result) => result,
			None => return Err(PrepareAssetError::RetryNextUpdate(extracted_asset)),
		};

		let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
			label: Some("material_bind_group"),
			layout: &pipeline.material_layout,
			entries: &[
				BindGroupEntry {
					binding: 0,
					resource: BindingResource::TextureView(color_map_0_view),
				},
				BindGroupEntry {
					binding: 1,
					resource: BindingResource::Sampler(color_map_0_sampler),
				},
			],
		});

		Ok(GpuMaterial { bind_group })
	}
}

impl Material {
	fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
		render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: Some("material_layout"),
			entries: &[
				BindGroupLayoutEntry {
					binding: 0,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Texture {
						sample_type: TextureSampleType::Float { filterable: true },
						view_dimension: TextureViewDimension::D2,
						multisampled: false,
					},
					// TODO: can we bind texures as an array to be fancy or is it not worth it?
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 1,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Sampler(SamplerBindingType::Filtering),
					count: None,
				},
			],
		})
	}
}

// TODO: name
#[derive(Bundle, Default)]
pub struct MeshBundle {
	pub mesh: Handle<Mesh>,
	pub material: Handle<Material>,

	pub transform: Transform,
	pub global_transform: GlobalTransform,

	pub visibility: Visibility,
	pub computed_visibility: ComputedVisibility,
}

type Draw = (
	SetItemPipeline,
	SetMeshViewBindGroup<0>,
	SetMeshBindGroup<1>,
	SetMaterialBindGroup<2>,
	DrawMesh,
);

struct SetMaterialBindGroup<const I: usize>;
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

// TODO: seperate file
pub struct Pipeline {
	mesh_pipeline: MeshPipeline,
	material_layout: BindGroupLayout,
	vertex_shader: Handle<Shader>,
	fragment_shader: Handle<Shader>,
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

			// TODO: at least the fragment shader should probably be from the material
			vertex_shader: asset_server.load("shader/mesh.wgsl"),
			fragment_shader: asset_server.load("shader/test.wgsl"),
		}
	}
}

impl SpecializedMeshPipeline for Pipeline {
	type Key = MeshPipelineKey;

	fn specialize(
		&self,
		key: Self::Key,
		layout: &MeshVertexBufferLayout,
	) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
		let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

		descriptor.vertex.shader = self.vertex_shader.clone();

		let fragment = descriptor.fragment.as_mut().unwrap();
		fragment.shader = self.fragment_shader.clone();

		descriptor.layout = Some(vec![
			self.mesh_pipeline.view_layout.clone(),
			self.mesh_pipeline.mesh_layout.clone(),
			self.material_layout.clone(),
		]);

		Ok(descriptor)
	}
}

#[allow(clippy::too_many_arguments)]
fn queue(
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
			let _material = match render_materials.get(material_handle) {
				Some(material) => material,
				None => continue,
			};

			let mesh = match render_meshes.get(mesh_handle) {
				Some(mesh) => mesh,
				None => continue,
			};

			let key = msaa_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
			let specialized_pipeline = pipelines
				.specialize(&mut pipeline_cache, &pipeline, key, &mesh.layout)
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
