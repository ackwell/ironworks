use bevy::{
	ecs::system::{lifetimeless::SRes, SystemParamItem},
	pbr::{Material, MaterialPipeline},
	reflect::TypeUuid,
	render::{
		render_asset::{PrepareAssetError, RenderAsset},
		render_resource::{
			BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor,
		},
		renderer::RenderDevice,
	},
};

#[derive(Clone, TypeUuid)]
#[uuid = "5f115bbc-7755-4a10-9f29-b078a84dbb10"]
pub struct BgMaterial {}

pub struct GpuBgMaterial {
	bind_group: BindGroup,
}

impl RenderAsset for BgMaterial {
	type ExtractedAsset = BgMaterial;
	type PreparedAsset = GpuBgMaterial;
	type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<BgMaterial>>);

	fn extract_asset(&self) -> Self::ExtractedAsset {
		self.clone()
	}

	fn prepare_asset(
		extracted_asset: Self::ExtractedAsset,
		(render_device, pipeline): &mut SystemParamItem<Self::Param>,
	) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
		let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
			label: None,
			layout: &pipeline.material_layout,
			entries: &[],
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
			entries: &[],
		})
	}
}
