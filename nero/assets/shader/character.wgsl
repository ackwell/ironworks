[[group(2), binding(4)]]
var normal_texture: texture_2d<f32>;
[[group(2), binding(5)]]
var normal_sampler: sampler;

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

	// Normal B is alpha
	if (normal.b <= 0.5) {
		discard;
	}

	// Normal W is colorset
	return vec4<f32>(normal.www, 1.0);
}
