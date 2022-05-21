// Copied from mesh.wgsl
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
// #ifdef VERTEX_TANGENTS
//     [[location(3)]] world_tangent: vec4<f32>;
// #endif
};

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
  var output_color = vec4<f32>(input.uv, 0.0, 1.0);
  return output_color;
}