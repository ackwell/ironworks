// Copied from mesh.wgsl
struct FragmentInput {
    [[builtin(front_facing)]] is_front: bool;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
// #ifdef VERTEX_TANGENTS
//     [[location(3)]] world_tangent: vec4<f32>;
// #endif
};

struct BgMaterial {};

// [[group(1), binding(0)]]
// var<uniform> uniform_data: BgMaterial;
[[group(1), binding(1)]]
var diffuse_texture: texture_2d<f32>;
[[group(1), binding(2)]]
var diffuse_sampler: sampler;

[[stage(fragment)]]
fn fragment(input: FragmentInput) -> [[location(0)]] vec4<f32> {
  var output_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
  output_color = output_color * textureSample(diffuse_texture, diffuse_sampler, input.uv);
  return output_color;
}