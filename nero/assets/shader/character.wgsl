[[group(2), binding(4)]]
var normal_texture: texture_2d<f32>;
[[group(2), binding(5)]]
var normal_sampler: sampler;
[[group(2), binding(6)]]
var color_set_texture: texture_2d<f32>;
[[group(2), binding(7)]]
var color_set_sampler: sampler;
[[group(2), binding(8)]]
var index_sampler: sampler;

// TODO: also uses SamplerNormal, which i _believe_ has colorset on W

struct FragmentInput {
	// [[builtin(front_facing)]] is_front: bool;
	// [[location(0)]] world_position: vec4<f32>;
	// [[location(1)]] world_normal: vec3<f32>;
	[[location(2)]] uv: vec4<f32>;
	// [[location(3)]] color: vec4<f32>;
};

[[stage(fragment)]]
fn fragment(input: FragmentInput) -> [[location(0)]] vec4<f32> {
	let normal = textureSample(normal_texture, normal_sampler, input.uv.xy);
	let index = textureSample(normal_texture, index_sampler, input.uv.xy);
	
	// Normal W is colorset
	// TODO: W channel of colorset is metadata for each of the fields
	let color_set = textureSample(color_set_texture, color_set_sampler, vec2<f32>(0.125, index.w));

	// also need to pull out of chara/common/texture/-tile_n|d.tex - maybe better to work on game shaders instead of going too far down that rabbit hole

	// Normal B is alpha
	if (normal.b <= 0.5) {
		discard;
	}

	return vec4<f32>(color_set.xyz, 1.0);
}
