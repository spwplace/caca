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

@group(0) @binding(0) var<storage, read> input_cells: array<u32>;
@group(0) @binding(1) var<storage, read_write> output_cells: array<u32>;
@group(0) @binding(2) var<storage, read> rules: array<u32>;
@group(0) @binding(3) var<uniform> uniforms: Uniforms;

fn get_cell_index(x: u32, y: u32) -> u32 {
    return y * uniforms.grid_width + x;
}

fn get_tile_index(x: u32, y: u32) -> vec2<u32> {
    let tile_x = x / uniforms.tile_width;
    let tile_y = y / uniforms.tile_height;
    return vec2<u32>(tile_x, tile_y);
}

fn wrap_within_tile(x: i32, y: i32, tile_x: u32, tile_y: u32) -> vec2<u32> {
    let tile_start_x = tile_x * uniforms.tile_width;
    let tile_start_y = tile_y * uniforms.tile_height;
    
    let local_x = ((x - i32(tile_start_x)) % i32(uniforms.tile_width) + i32(uniforms.tile_width)) % i32(uniforms.tile_width);
    let local_y = ((y - i32(tile_start_y)) % i32(uniforms.tile_height) + i32(uniforms.tile_height)) % i32(uniforms.tile_height);
    
    return vec2<u32>(tile_start_x + u32(local_x), tile_start_y + u32(local_y));
}

fn get_neighbor_config(x: u32, y: u32, tile_x: u32, tile_y: u32) -> u32 {
    var config: u32 = 0u;
    var bit: u32 = 0u;
    
    for (var dy: i32 = -1; dy <= 1; dy++) {
        for (var dx: i32 = -1; dx <= 1; dx++) {
            let wrapped = wrap_within_tile(i32(x) + dx, i32(y) + dy, tile_x, tile_y);
            let cell_state = input_cells[get_cell_index(wrapped.x, wrapped.y)];
            config |= (cell_state & 1u) << bit;
            bit++;
        }
    }
    
    return config;
}

fn count_neighbors(x: u32, y: u32, tile_x: u32, tile_y: u32) -> u32 {
    var count: u32 = 0u;
    
    for (var dy: i32 = -1; dy <= 1; dy++) {
        for (var dx: i32 = -1; dx <= 1; dx++) {
            if (dx == 0 && dy == 0) { continue; }
            let wrapped = wrap_within_tile(i32(x) + dx, i32(y) + dy, tile_x, tile_y);
            count += input_cells[get_cell_index(wrapped.x, wrapped.y)] & 1u;
        }
    }
    
    return count;
}

fn apply_general_rule(config: u32, tile_idx: u32) -> u32 {
    let rule_base = tile_idx * 16u;
    let word_idx = config / 32u;
    let bit_idx = config % 32u;
    return (rules[rule_base + word_idx] >> bit_idx) & 1u;
}

fn apply_totalistic_rule(center: u32, neighbor_count: u32, tile_idx: u32) -> u32 {
    let rule_word = rules[tile_idx * 16u];
    if (center == 1u) {
        return (rule_word >> neighbor_count) & 1u;
    } else {
        return (rule_word >> (neighbor_count + 9u)) & 1u;
    }
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;
    
    if (x >= uniforms.grid_width || y >= uniforms.grid_height) {
        return;
    }
    
    let tile = get_tile_index(x, y);
    let tile_idx = tile.y * uniforms.tile_cols + tile.x;
    let cell_idx = get_cell_index(x, y);
    
    var next_state: u32;
    
    if (uniforms.use_totalistic == 1u) {
        let center = input_cells[cell_idx] & 1u;
        let neighbors = count_neighbors(x, y, tile.x, tile.y);
        next_state = apply_totalistic_rule(center, neighbors, tile_idx);
    } else {
        let config = get_neighbor_config(x, y, tile.x, tile.y);
        next_state = apply_general_rule(config, tile_idx);
    }
    
    output_cells[cell_idx] = next_state;
}
