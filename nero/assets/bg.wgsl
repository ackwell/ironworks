#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

[[group(1), binding(0)]]
var diffuse1_texture: texture_2d<f32>;
[[group(1), binding(1)]]
var diffuse1_sampler: sampler;
[[group(1), binding(2)]]
var diffuse2_texture: texture_2d<f32>;
[[group(1), binding(3)]]
var diffuse2_sampler: sampler;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh;

struct Vertex {
	[[location(0)]] position: vec3<f32>;
	[[location(1)]] normal: vec3<f32>;
	[[location(2)]] uv: vec4<f32>;
	[[location(3)]] color: vec4<f32>;
};

struct VertexOutput {
	[[builtin(position)]] clip_position: vec4<f32>;
	[[location(0)]] world_position: vec4<f32>;
	[[location(1)]] world_normal: vec3<f32>;
	[[location(2)]] uv: vec4<f32>;
	[[location(3)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
	var out: VertexOutput;
	out.world_position = mesh.model * vec4<f32>(vertex.position, 1.0);
	out.world_normal = mat3x3<f32>(
		mesh.inverse_transpose_model[0].xyz,
		mesh.inverse_transpose_model[1].xyz,
		mesh.inverse_transpose_model[2].xyz
	) * vertex.normal;

	out.uv = vertex.uv;
	out.clip_position = view.view_proj * out.world_position;
	out.color = vertex.color;

	return out;
}

struct FragmentInput {
	[[builtin(front_facing)]] is_front: bool;
	[[location(0)]] world_position: vec4<f32>;
	[[location(1)]] world_normal: vec3<f32>;
	[[location(2)]] uv: vec4<f32>;
	[[location(3)]] color: vec4<f32>;
};

[[stage(fragment)]]
fn fragment(input: FragmentInput) -> [[location(0)]] vec4<f32> {
	// TODO: clamping of the uv can likely be done at a texture level.
	let diffuse1 = textureSample(diffuse1_texture, diffuse1_sampler, abs(input.uv.xy) % 1.0);
	let diffuse2 = textureSample(diffuse2_texture, diffuse2_sampler, abs(input.uv.zw) % 1.0);
	// TODO: There seems to be some alpha on diffuse2 that causes holes in solid materials - should this be additive rather than mixing?
	let diffuse = mix(diffuse1, diffuse2, input.color.w);

	// TODO: can this be done at a material level?
	if (diffuse.w <= 0.5) {
		discard;
	}

	// TODO: All the other important things
	return diffuse;
}
