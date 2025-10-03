struct Data { width: u32, height: u32, scale: f32 };
struct Seed { data: array<u32> };
struct Vectors { data: array<vec2<f32>> };
struct Output { data: array<f32> };

@group(0) @binding(0) var<uniform> data: Data;
@group(0) @binding(1) var<storage, read> seed: Seed;
@group(0) @binding(2) var<storage, read> vectors: Vectors;
@group(0) @binding(3) var<storage, read_write> output: Output;

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

fn fade(num: f32) -> f32 {
    return (6.0 * pow(num, 5.0)) -
           (15.0 * pow(num, 4.0)) +
           (10.0 * pow(num, 3.0));
}

fn from_point(x: i32, y: i32) -> vec2<f32> {
    let x_bwand = u32(x) & 255u;
    let y_bwand = u32(y) & 255u;

    let hash = seed.data[(seed.data[x_bwand] + y_bwand) & 255u];
    let vector_index = hash % 16u;

    return vectors.data[vector_index];
}

fn from_sample(x: f32, y: f32) -> f32 {
    let x0 = i32(floor(x));
    let y0 = i32(floor(y));

    let g00 = from_point(x0,     y0);
    let g10 = from_point(x0 + 1, y0);
    let g01 = from_point(x0,     y0 + 1);
    let g11 = from_point(x0 + 1, y0 + 1);

    let dx = x - f32(x0);
    let dy = y - f32(y0);

    let n00 = dot(g00, vec2<f32>(dx,       dy));
    let n10 = dot(g10, vec2<f32>(dx - 1.0, dy));
    let n01 = dot(g01, vec2<f32>(dx,       dy - 1.0));
    let n11 = dot(g11, vec2<f32>(dx - 1.0, dy - 1.0));

    let nx0 = lerp(n00, n10, fade(dx));
    let nx1 = lerp(n01, n11, fade(dx));

    return lerp(nx0, nx1, fade(dy));
}

fn from_fractal(x: f32, y: f32, octaves: u32, lacunarity: f32, persistence: f32) -> f32 {
    var total: f32 = 0.0;
    var frequency: f32 = 1.0;
    var amplitude: f32 = 1.0;
    var max_amplitude: f32 = 0.0;

    for (var i: u32 = 0u; i < octaves; i = i + 1u) {
        total = total + from_sample(x * frequency, y * frequency) * amplitude;
        max_amplitude = max_amplitude + amplitude;
        frequency = frequency * lacunarity;
        amplitude = amplitude * persistence;
    }

    return total / max_amplitude;
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let CHUNK_SIZE = 32u;
    let CHUNK_AREA = 1024u;

    let gx: u32 = gid.x;
    let gy: u32 = gid.y;

    if (gx >= data.width || gy >= data.height) {
        return;
    }

    let chunks_x: u32 = data.width / CHUNK_SIZE;
    let chunks_y: u32 = data.height / CHUNK_SIZE;

    let chunk_x: u32 = gx / CHUNK_SIZE;
    let chunk_y: u32 = gy / CHUNK_SIZE;
    let local_x: u32 = gx - chunk_x * CHUNK_SIZE;
    let local_y: u32 = gy - chunk_y * CHUNK_SIZE;

    let chunk_index: u32 = chunk_y * chunks_x + chunk_x;

    let within_chunk_index: u32 = local_y * CHUNK_SIZE + local_x;
    let out_index: u32 = chunk_index * CHUNK_AREA + within_chunk_index;

    let sx: f32 = f32(gx) * data.scale;
    let sy: f32 = f32(gy) * data.scale;

    let noise: f32 = from_fractal(sx, sy, 4u, 2.0, 0.5);

    output.data[out_index] = noise;
}
