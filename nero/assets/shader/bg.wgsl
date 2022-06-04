[[group(2), binding(0)]]
var color_map_0_texture: texture_2d<f32>;
[[group(2), binding(1)]]
var color_map_0_sampler: sampler;
[[group(2), binding(2)]]
var color_map_1_texture: texture_2d<f32>;
[[group(2), binding(3)]]
var color_map_1_sampler: sampler;

struct FragmentInput {
	// [[builtin(front_facing)]] is_front: bool;
	// [[location(0)]] world_position: vec4<f32>;
	// [[location(1)]] world_normal: vec3<f32>;
	[[location(2)]] uv: vec4<f32>;
	[[location(3)]] color: vec4<f32>;
};

[[stage(fragment)]]
fn fragment(input: FragmentInput) -> [[location(0)]] vec4<f32> {
	let color_map_0 = textureSample(color_map_0_texture, color_map_0_sampler, input.uv.xy);
	let color_map_1 = textureSample(color_map_1_texture, color_map_1_sampler, input.uv.zw);

	let color_map = mix(color_map_0, color_map_1, input.color.w);
	
	if (color_map.w <= 0.5) {
		discard;
	}

	// TODO: All the other important things
	return color_map;
}
