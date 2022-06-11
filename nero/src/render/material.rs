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
	utils::HashMap,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MaterialKind {
	Unknown,

	Bg,
	Character,
	Skin,
}

#[derive(Clone, TypeUuid)]
#[uuid = "317a2fbb-6fb4-4bbd-b480-1d5942345cc0"]
pub struct Material {
	pub kind: MaterialKind,

	// ??
	pub color_set: Option<Handle<Image>>,

	pub samplers: HashMap<u32, Handle<Image>>,
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
		// Helper macro to make texture reading less painful. This is a macro rather than a closure to make returning simpler.
		macro_rules! get_texture {
			($handle:expr) => {
				match pipeline.mesh_pipeline.get_image_texture(images, $handle) {
					Some(result) => result,
					None => return Err(PrepareAssetError::RetryNextUpdate(extracted_asset)),
				}
			};
		}

		// TODO: work out how i want to handle the sampler IDs without like literally hardcoding them.
		// TODO: I really should be specialising for this in some way, binding everything is pants on head.
		let (color_map_0_view, color_map_0_sampler) =
			get_texture!(&extracted_asset.samplers.get(&0x1E6FEF9C).cloned());
		let (color_map_1_view, color_map_1_sampler) =
			get_texture!(&extracted_asset.samplers.get(&0x6968DF0A).cloned());
		let (diffuse_view, diffuse_sampler) =
			get_texture!(&extracted_asset.samplers.get(&0x115306BE).cloned());
		let (normal_view, normal_sampler) =
			get_texture!(&extracted_asset.samplers.get(&0x0C5EC1F1).cloned());

		// Colorset stuff
		let (color_set_view, color_set_sampler) = get_texture!(&extracted_asset.color_set);
		// TODO: this is incredibly janky. building a custom sampler here for non-blurry lookup in the normal. realistically, the game shaders actually bind the normal texture twice, once as "Normal", once as "Index", with the latter having the seperate sampler. This is relying on the default sampler using nearest filtering.
		let index_sampler = &render_device.create_sampler(&Default::default());

		// TODO: work out how to generate the bind group + layout because this is getting dumb
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
				BindGroupEntry {
					binding: 2,
					resource: BindingResource::TextureView(color_map_1_view),
				},
				BindGroupEntry {
					binding: 3,
					resource: BindingResource::Sampler(color_map_1_sampler),
				},
				BindGroupEntry {
					binding: 4,
					resource: BindingResource::TextureView(normal_view),
				},
				BindGroupEntry {
					binding: 5,
					resource: BindingResource::Sampler(normal_sampler),
				},
				BindGroupEntry {
					binding: 6,
					resource: BindingResource::TextureView(color_set_view),
				},
				BindGroupEntry {
					binding: 7,
					resource: BindingResource::Sampler(color_set_sampler),
				},
				BindGroupEntry {
					binding: 8,
					resource: BindingResource::Sampler(index_sampler),
				},
				BindGroupEntry {
					binding: 9,
					resource: BindingResource::TextureView(diffuse_view),
				},
				BindGroupEntry {
					binding: 10,
					resource: BindingResource::Sampler(diffuse_sampler),
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
			MaterialKind::Unknown => "shader/placeholder.wgsl",
			MaterialKind::Bg => "shader/bg.wgsl",
			MaterialKind::Character => "shader/character.wgsl",
			MaterialKind::Skin => "shader/skin.wgsl",
			#[allow(unreachable_patterns)]
			other => {
				warn!("Unhandled material kind: {other:?}");
				"shader/placeholder.wgsl"
			}
		}
	}

	// TODO: if this needs access to the key, it'll need to be moved to the specialise step in the pipeline. that could be a bit fiddly, as the layout is required to build the bind group, which is currently done in prepare - which needs to execute before specialisation. building the layout can probably be moved to a prepare step, but will need to be cached to prevent it being generated every frame. Or not? Reading through the source, prepare is only called once per extract, and extraction of render assets only occurs when they change. It effectively is caching already.
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
				BindGroupLayoutEntry {
					binding: 2,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Texture {
						sample_type: TextureSampleType::Float { filterable: true },
						view_dimension: TextureViewDimension::D2,
						multisampled: false,
					},
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 3,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Sampler(SamplerBindingType::Filtering),
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 4,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Texture {
						sample_type: TextureSampleType::Float { filterable: true },
						view_dimension: TextureViewDimension::D2,
						multisampled: false,
					},
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 5,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Sampler(SamplerBindingType::Filtering),
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 6,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Texture {
						sample_type: TextureSampleType::Float { filterable: true },
						view_dimension: TextureViewDimension::D2,
						multisampled: false,
					},
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 7,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Sampler(SamplerBindingType::Filtering),
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 8,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Sampler(SamplerBindingType::Filtering),
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 9,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Texture {
						sample_type: TextureSampleType::Float { filterable: true },
						view_dimension: TextureViewDimension::D2,
						multisampled: false,
					},
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 10,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Sampler(SamplerBindingType::Filtering),
					count: None,
				},
			],
		})
	}
}
