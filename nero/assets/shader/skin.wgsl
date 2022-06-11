[[group(2), binding(9)]]
var diffuse_texture: texture_2d<f32>;
[[group(2), binding(10)]]
var diffuse_sampler: sampler;


struct FragmentInput {
	// [[builtin(front_facing)]] is_front: bool;
	// [[location(0)]] world_position: vec4<f32>;
	// [[location(1)]] world_normal: vec3<f32>;
	[[location(2)]] uv: vec4<f32>;
	// [[location(3)]] color: vec4<f32>;
};

[[stage(fragment)]]
fn fragment(input: FragmentInput) -> [[location(0)]] vec4<f32> {
	return textureSample(diffuse_texture, diffuse_sampler, input.uv.xy);
}
