use bevy::{
	ecs::system::{lifetimeless::SRes, SystemParamItem},
	pbr::{Material, MaterialPipeline},
	prelude::*,
	reflect::TypeUuid,
	render::{
		render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
		render_resource::{
			std140::{AsStd140, Std140},
			BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
			BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
			BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages, SamplerBindingType,
			ShaderStages, TextureSampleType, TextureViewDimension,
		},
		renderer::RenderDevice,
	},
};

use super::plugin::BG_SHADER_HANDLE;

#[derive(Clone, TypeUuid)]
#[uuid = "5f115bbc-7755-4a10-9f29-b078a84dbb10"]
pub struct BgMaterial {
	pub diffuse: Handle<Image>,
}

// #[derive(AsStd140)]
// struct BgMaterialUniformData {}

pub struct GpuBgMaterial {
	bind_group: BindGroup,
}

impl RenderAsset for BgMaterial {
	type ExtractedAsset = BgMaterial;
	type PreparedAsset = GpuBgMaterial;
	type Param = (
		SRes<RenderDevice>,
		SRes<MaterialPipeline<BgMaterial>>,
		SRes<RenderAssets<Image>>,
	);

	fn extract_asset(&self) -> Self::ExtractedAsset {
		self.clone()
	}

	fn prepare_asset(
		extracted_asset: Self::ExtractedAsset,
		(render_device, pipeline, images): &mut SystemParamItem<Self::Param>,
	) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
		// let uniform_data = BgMaterialUniformData {};

		// let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
		// 	label: None,
		// 	contents: uniform_data.as_std140().as_bytes(),
		// 	usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
		// });

		let (diffuse_view, diffuse_sampler) = if let Some(result) = pipeline
			.mesh_pipeline
			.get_image_texture(images, &Some(extracted_asset.diffuse.clone()))
		{
			result
		} else {
			return Err(PrepareAssetError::RetryNextUpdate(extracted_asset));
		};

		let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
			label: None,
			layout: &pipeline.material_layout,
			entries: &[
				// BindGroupEntry {
				// 	binding: 0,
				// 	resource: buffer.as_entire_binding(),
				// },
				BindGroupEntry {
					binding: 1,
					resource: BindingResource::TextureView(diffuse_view),
				},
				BindGroupEntry {
					binding: 2,
					resource: BindingResource::Sampler(diffuse_sampler),
				},
			],
		});
		Ok(GpuBgMaterial { bind_group })
	}
}

impl Material for BgMaterial {
	fn bind_group(material: &Self::PreparedAsset) -> &BindGroup {
		&material.bind_group
	}

	fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
		render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: None,
			entries: &[
				// BindGroupLayoutEntry {
				// 	binding: 0,
				// 	visibility: ShaderStages::FRAGMENT,
				// 	ty: BindingType::Buffer {
				// 		ty: BufferBindingType::Uniform,
				// 		has_dynamic_offset: false,
				// 		min_binding_size: BufferSize::new(
				// 			u64::try_from(BgMaterialUniformData::std140_size_static()).unwrap(),
				// 		),
				// 	},
				// 	count: None,
				// },
				BindGroupLayoutEntry {
					binding: 1,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Texture {
						sample_type: TextureSampleType::Float { filterable: true },
						view_dimension: TextureViewDimension::D2,
						multisampled: false,
					},
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 2,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Sampler(SamplerBindingType::Filtering),
					count: None,
				},
			],
		})
	}

	fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
		Some(BG_SHADER_HANDLE.typed())
	}
}
