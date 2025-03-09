struct FogOfWar {
    visible_points: array<vec2<f32>, 1024>, // Adjust size based on your needs
    point_count: u32,
    maze_size: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> fog: FogOfWar;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(vertex.position, 1.0);
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Convert UV coordinates to maze coordinates
    let maze_coord = in.uv * fog.maze_size;

    // Check if this point is visible
    var is_visible = false;
    for (var i: u32 = 0u; i < fog.point_count; i = i + 1u) {
        let visible_point = fog.visible_points[i];
        // Adjust distance threshold based on your needs
        if (distance(maze_coord, visible_point) < 0.5) {
            is_visible = true;
            break;
        }
    }

    // If visible, return clear color (transparent)
    // If not visible, return fog color (dark)
    if (is_visible) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0); // Transparent
    } else {
        return vec4<f32>(0.0, 0.0, 0.0, 0.8); // Semi-transparent black fog
    }
}
