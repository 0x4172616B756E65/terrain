struct Size { width: u32, height: u32 };
struct Seed { data: array<u32> };
struct Vectors { data: array<vec2<f32>> };
struct Output { data: array<f32> };

@group(0) @binding(0) var<uniform> size: Size;
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
    let x_idx = u32(x % 256);
    let y_idx = u32(y % 256);
    let seed_value = seed.data[(seed.data[x_idx] + y_idx) % 256u];
    let index = seed_value % 8u;
    return vectors.data[index];
}

fn from_sample(x: f32, y: f32) -> f32 {
    let x0 = i32(floor(x));
    let y0 = i32(floor(y));

    let g00 = from_point(x0,     y0);
    let g10 = from_point(x0 + 1, y0);
    let g01 = from_point(x0,     y0 + 1);
    let g11 = from_point(x0 + 1, y0 + 1);

    let dx = x - f32(x0) + 0.1;
    let dy = y - f32(y0) + 0.1;

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
    if (gid.x >= 32u || gid.y >= 32u) { return; }
    let gid_x = gid.x;
    let gid_y = gid.y;
    let idx = gid_y * size.width + gid_x;

    if gid_x < size.width && gid_y < size.height {
    output.data[idx] = from_fractal(f32(gid_x), f32(gid_y), 5u, 2.0, 0.5);
}
}

