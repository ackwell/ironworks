#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

[[group(2), binding(0)]]
var<uniform> mesh: Mesh;

struct Vertex {
		[[location(0)]] position: vec3<f32>;
		[[location(1)]] normal: vec3<f32>;
		[[location(2)]] uv: vec4<f32>;
// #ifdef VERTEX_TANGENTS
//     [[location(3)]] tangent: vec4<f32>;
// #endif
// #ifdef SKINNED
//     [[location(4)]] joint_indices: vec4<u32>;
//     [[location(5)]] joint_weights: vec4<f32>;
// #endif
		[[location(3)]] color: vec4<f32>;
};

struct VertexOutput {
		[[builtin(position)]] clip_position: vec4<f32>;
		[[location(0)]] world_position: vec4<f32>;
		[[location(1)]] world_normal: vec3<f32>;
		[[location(2)]] uv: vec4<f32>;
// #ifdef VERTEX_TANGENTS
//     [[location(3)]] world_tangent: vec4<f32>;
// #endif
		[[location(3)]] color: vec4<f32>;
};

// Copied from mesh.wgsl
struct FragmentInput {
		[[builtin(front_facing)]] is_front: bool;
		[[location(0)]] world_position: vec4<f32>;
		[[location(1)]] world_normal: vec3<f32>;
		[[location(2)]] uv: vec4<f32>;
// #ifdef VERTEX_TANGENTS
//     [[location(3)]] world_tangent: vec4<f32>;
// #endif
		[[location(3)]] color: vec4<f32>;
};

struct BgMaterial {};

// [[group(1), binding(0)]]
// var<uniform> uniform_data: BgMaterial;
[[group(1), binding(1)]]
var diffuse_texture: texture_2d<f32>;
[[group(1), binding(2)]]
var diffuse_sampler: sampler;

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
	
		var out: VertexOutput;
// #ifdef SKINNED
//     var model = skin_model(vertex.joint_indices, vertex.joint_weights);
//     out.world_position = model * vec4<f32>(vertex.position, 1.0);
//     out.world_normal = skin_normals(model, vertex.normal);
// // #ifdef VERTEX_TANGENTS
// //     out.world_tangent = skin_tangents(model, vertex.tangent);
// // #endif
// #else
		out.world_position = mesh.model * vec4<f32>(vertex.position, 1.0);
		out.world_normal = mat3x3<f32>(
				mesh.inverse_transpose_model[0].xyz,
				mesh.inverse_transpose_model[1].xyz,
				mesh.inverse_transpose_model[2].xyz
		) * vertex.normal;
// #ifdef VERTEX_TANGENTS
//     out.world_tangent = vec4<f32>(
//         mat3x3<f32>(
//             mesh.model[0].xyz,
//             mesh.model[1].xyz,
//             mesh.model[2].xyz
//         ) * vertex.tangent.xyz,
//         vertex.tangent.w
//     );
// #endif
// #endif

		// it me
		out.color = vertex.color;

		out.uv = vertex.uv;
		out.clip_position = view.view_proj * out.world_position;

		return out;
}

[[stage(fragment)]]
fn fragment(input: FragmentInput) -> [[location(0)]] vec4<f32> {
	var output_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
  // output_color = output_color * textureSample(diffuse_texture, diffuse_sampler, abs(input.uv.xy) % 1.0);
//   var output_color =   vec4<f32>(abs(input.uv.xy) % 1.0, 0.0, 1.0);
	
	
// #ifdef VERTEX_COLOR
//   var output_color =   vec4<f32>(abs(input.uv.xy) % 1.0, 0.0, 1.0);
	var output_color = output_color * input.color.wwww;
// 	// output_color = normalize(output_color);

	return output_color;
// #else
	
// 	return vec4<f32>(1.0, 0.0, 1.0, 1.0);
// #endif
}
