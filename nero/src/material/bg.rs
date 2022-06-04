use bevy::{
	ecs::system::{lifetimeless::SRes, SystemParamItem},
	pbr::{Material, MaterialPipeline},
	prelude::*,
	reflect::TypeUuid,
	render::{
		mesh::MeshVertexBufferLayout,
		render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
		render_resource::{
			BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
			BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
			RenderPipelineDescriptor, SamplerBindingType, ShaderStages,
			SpecializedMeshPipelineError, TextureSampleType, TextureViewDimension,
		},
		renderer::RenderDevice,
	},
};

use crate::render::{ATTRIBUTE_COLOR, ATTRIBUTE_UV_4};

#[derive(Clone, TypeUuid)]
#[uuid = "5f115bbc-7755-4a10-9f29-b078a84dbb10"]
pub struct BgMaterial {
	pub diffuse1: Option<Handle<Image>>,
	pub diffuse2: Option<Handle<Image>>,
}

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
		let (diffuse1_view, diffuse1_sampler) = if let Some(result) = pipeline
			.mesh_pipeline
			.get_image_texture(images, &extracted_asset.diffuse1)
		{
			result
		} else {
			return Err(PrepareAssetError::RetryNextUpdate(extracted_asset));
		};

		let (diffuse2_view, diffuse2_sampler) = if let Some(result) = pipeline
			.mesh_pipeline
			.get_image_texture(images, &extracted_asset.diffuse2)
		{
			result
		} else {
			return Err(PrepareAssetError::RetryNextUpdate(extracted_asset));
		};

		let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
			label: None,
			layout: &pipeline.material_layout,
			entries: &[
				BindGroupEntry {
					binding: 0,
					resource: BindingResource::TextureView(diffuse1_view),
				},
				BindGroupEntry {
					binding: 1,
					resource: BindingResource::Sampler(diffuse1_sampler),
				},
				BindGroupEntry {
					binding: 2,
					resource: BindingResource::TextureView(diffuse2_view),
				},
				BindGroupEntry {
					binding: 3,
					resource: BindingResource::Sampler(diffuse2_sampler),
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
				// TODO: can i array this? at all? how do arrays work?
				BindGroupLayoutEntry {
					binding: 0,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Texture {
						sample_type: TextureSampleType::Float { filterable: true },
						view_dimension: TextureViewDimension::D2,
						multisampled: false,
					},
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
			],
		})
	}

	fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
		Some(asset_server.load("bg.wgsl"))
	}

	fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
		Some(asset_server.load("bg.wgsl"))
	}

	fn specialize(
		_pipeline: &MaterialPipeline<Self>,
		descriptor: &mut RenderPipelineDescriptor,
		layout: &MeshVertexBufferLayout,
	) -> Result<(), SpecializedMeshPipelineError> {
		let vertex_attributes = vec![
			Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
			Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
			ATTRIBUTE_UV_4.at_shader_location(2),
			ATTRIBUTE_COLOR.at_shader_location(3),
		];

		let vertex_layout = layout.get_layout(&vertex_attributes)?;

		descriptor.vertex.buffers = vec![vertex_layout];

		Ok(())
	}
}
