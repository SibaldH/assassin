@group(0) @binding(0) var<uniform> resolution: vec2<f32>;
@group(0) @binding(1) var<uniform> player_pos: vec2<f32>;
@group(0) @binding(2) var<uniform> light_radius: f32;

@fragment
fn fragment(
    @location(0) uv: vec2<f32>,
    @builtin(position) position: vec4<f32>,
) -> @location(0) vec4<f32> {
    let pixel_coords = position.xy;
    let dist = distance(pixel_coords, player_pos);

    // Create a smooth falloff for the light
    let light = smoothstep(light_radius, light_radius * 0.8, dist);
    let alpha = mix(0.95, 0.0, light); // Fade from dark to transparent

    return vec4<f32>(0.0, 0.0, 0.0, alpha);
}
