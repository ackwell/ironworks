struct FragmentInput {
	[[builtin(front_facing)]] is_front: bool;
	[[location(0)]] world_position: vec4<f32>;
	[[location(1)]] world_normal: vec3<f32>;
	[[location(2)]] uv: vec4<f32>;
	// [[location(3)]] color: vec4<f32>;
};

[[stage(fragment)]]
fn fragment(input: FragmentInput) -> [[location(0)]] vec4<f32> {
	// TODO: All the other important things
	return vec4<f32>(0.5, 0.5, 0.5, 1.0);
}
