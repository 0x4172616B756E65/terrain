use std::u64;

use bevy::math::ops::floor;
use rand::{self, rand_core::le, rngs::StdRng, Rng, SeedableRng};
use tracing::info;

const VECTORS: [Vector; 8] = [
    Vector { x: 1.0, y: 0.0 },
    Vector { x: 0.0, y: 1.0 },
    Vector { x: -1.0, y: 0.0 },
    Vector { x: 0.0, y: -1.0 },
    Vector { x: 0.70710677, y: 0.70710677 },
    Vector { x: -0.70710677, y: 0.70710677 },
    Vector { x: 0.70710677, y: -0.70710677 },
    Vector { x: -0.70710677, y: -0.70710677 },
];

#[derive(Debug, Clone, Copy)]
pub struct Seed {
    pub table: [usize; 512]
}

impl Seed {
    pub fn new(seed: u64) -> Self {
        let mut table: [usize; 512] = [0; 512];
        let mut rng = StdRng::seed_from_u64(seed);

        //Fill the table
        for i in 0..256 {
            table[i] = i;
        }

        //Permutate the table
        for i in 0..256 {
            let j = rng.random_range(i..256);
            let t = table[j];

            table[j] = table[i];
            table[i] = t;
        } 

        //Duplicate the table
        for i in 0..256 {
            table[i + 256] = table[i];
        }

        Seed { table }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }
    
    pub fn dot(vector_a: Vector, vector_b: Vector) -> f32 {
        vector_a.x * vector_b.x +
        vector_a.y * vector_b.y
    }

}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
#[derive(Debug)]
pub struct Perlin;

impl Perlin {
    pub fn new() -> Self { Perlin }

    pub fn from_point(&self, seed: Seed, x: usize, y: usize) -> Vector {
        let index = seed.table[(seed.table[x % 256] + y % 256) % 256] % VECTORS.len(); 
        VECTORS[index]
    }

    pub fn from_sample(&self, seed: Seed, x: f32, y: f32) -> f32 {
        let x0 = floor(x) as usize;
        let y0 = floor(y) as usize;

        let g00 = self.from_point(seed, x0, y0);
        let g10 = self.from_point(seed, x0+1, y0);
        let g01 = self.from_point(seed, x0, y0+1);
        let g11 = self.from_point(seed, x0+1, y0+1);

        let dx = x - x0 as f32;
        let dy = y - y0 as f32;

        let offset_g00 = Vector::new(dx, dy);
        let offset_g10 = Vector::new(dx - 1., dy);
        let offset_g01 = Vector::new(dx, dy - 1.);
        let offset_g11 = Vector::new(dx - 1., dy - 1.);

        let n00 = Vector::dot(g00, offset_g00);
        let n10 = Vector::dot(g10, offset_g10);
        let n01 = Vector::dot(g01, offset_g01);
        let n11 = Vector::dot(g11, offset_g11);

        let fade_x = fade(dx);
        let fade_y = fade(dy);
        
        let nx0 = lerp(n00, n10, fade_x);
        let nx1 = lerp(n01, n11, fade_x);

        lerp(nx0, nx1, fade_y)
    }
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn fade(num: f32) -> f32 {
    6. * num.powf(5.) -
    15. * num.powf(4.) +
    10. * num.powf(3.)
}
