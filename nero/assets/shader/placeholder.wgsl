struct FragmentInput {
	[[builtin(front_facing)]] is_front: bool;
};

[[stage(fragment)]]
fn fragment(input: FragmentInput) -> [[location(0)]] vec4<f32> {
	// TODO: clamping of the uv can likely be done at a texture level.
	return vec4<f32>(0.5,0.0,0.0,1.0);
}
