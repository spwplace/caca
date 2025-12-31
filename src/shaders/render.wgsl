struct Uniforms {
    grid_width: u32,
    grid_height: u32,
    tile_cols: u32,
    tile_rows: u32,
    tile_width: u32,
    tile_height: u32,
    use_totalistic: u32,
    _padding: u32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var<storage, read> cells: array<u32>;
@group(0) @binding(1) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, 1.0)
    );
    
    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 0.0)
    );
    
    var output: VertexOutput;
    output.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    output.uv = uvs[vertex_index];
    return output;
}

fn hash_tile(tile_idx: u32) -> vec3<f32> {
    let h1 = fract(sin(f32(tile_idx) * 12.9898) * 43758.5453);
    let h2 = fract(sin(f32(tile_idx) * 78.233) * 43758.5453);
    let h3 = fract(sin(f32(tile_idx) * 45.164) * 43758.5453);
    return vec3<f32>(h1, h2, h3);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_x = u32(input.uv.x * f32(uniforms.grid_width));
    let pixel_y = u32(input.uv.y * f32(uniforms.grid_height));
    
    let clamped_x = min(pixel_x, uniforms.grid_width - 1u);
    let clamped_y = min(pixel_y, uniforms.grid_height - 1u);
    
    let cell_idx = clamped_y * uniforms.grid_width + clamped_x;
    let cell_state = cells[cell_idx];
    
    let tile_x = clamped_x / uniforms.tile_width;
    let tile_y = clamped_y / uniforms.tile_height;
    let tile_idx = tile_y * uniforms.tile_cols + tile_x;
    
    let tile_color = hash_tile(tile_idx);
    
    let local_x = clamped_x % uniforms.tile_width;
    let local_y = clamped_y % uniforms.tile_height;
    let on_border = local_x == 0u || local_y == 0u;
    
    if (on_border) {
        return vec4<f32>(0.2, 0.2, 0.2, 1.0);
    }
    
    if (cell_state == 1u) {
        return vec4<f32>(tile_color, 1.0);
    } else {
        return vec4<f32>(tile_color * 0.1, 1.0);
    }
}
