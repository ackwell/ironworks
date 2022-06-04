use bevy::{
	ecs::system::{lifetimeless::SRes, SystemParamItem},
	prelude::*,
	reflect::TypeUuid,
	render::{
		render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
		render_resource::{
			BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
			BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
			SamplerBindingType, ShaderStages, TextureSampleType, TextureViewDimension,
		},
		renderer::RenderDevice,
	},
};

use super::pipeline::Pipeline;

// TODO: if this grows much, move into seperate file
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MaterialKind {
	Bg,
	// TODO: this is purely testing cruft - will need to be replaced by the rest of the real shaders
	Other,
}

#[derive(Clone, TypeUuid)]
#[uuid = "317a2fbb-6fb4-4bbd-b480-1d5942345cc0"]
pub struct Material {
	pub kind: MaterialKind,

	// TODO: the rest. if ending up with shaders from the game files, this will need revisiting.
	//       ... even if not using shaders from game files, this might be best represented by a map, perhaps - given the game uses unique keys already.
	pub color_map_0: Option<Handle<Image>>,
}

pub struct GpuMaterial {
	pub kind: MaterialKind,
	pub bind_group: BindGroup,
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

		Ok(GpuMaterial {
			kind: extracted_asset.kind,
			bind_group,
		})
	}
}

// TODO: maybe move this stuff into a material trait so i can really _really_ just be a copypasta of bevy's impl lmao
pub type MaterialKey = MaterialKind;

impl Material {
	// TODO: would the fq trait be avoidable if mtrl was a trait?
	pub fn key(prepared: &<Self as RenderAsset>::PreparedAsset) -> MaterialKey {
		prepared.kind.clone()
	}

	// TODO: should this recieve the asset server or nah?
	pub fn fragment_shader(key: MaterialKey) -> &'static str {
		match key {
			MaterialKind::Bg => "shader/bg.wgsl",
			MaterialKind::Other => "shader/test.wgsl",
		}
	}

	pub fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
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
