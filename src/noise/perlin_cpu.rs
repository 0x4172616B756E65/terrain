#[cfg(feature = "debug")]
use std::time::Instant;

use bevy::{ecs::resource::Resource, math::ops::floor};
use rand::{self, rngs::StdRng, seq::SliceRandom, SeedableRng};
#[cfg(feature = "debug")]
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

#[derive(Debug, Clone, Copy, Resource)]
pub struct PerlinCPU {
    pub seed: [u8; 512], 
    pub scale: f32,

    pub octaves: usize,
    pub lacunarity: f32,
    pub persistence: f32,
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
   
    #[inline]
    pub fn dot(&self, other: Vector) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


impl PerlinCPU {
    pub fn new(seed: u64, scale: f32, octaves: usize, lacunarity: f32, persistence: f32) -> Self {
        #[cfg(feature = "debug")]
        let start = Instant::now();

        let mut table_256: [u8; 256] = (0..=255u8).collect::<Vec<u8>>().try_into().unwrap();
        let mut rng = StdRng::seed_from_u64(seed);
        table_256.shuffle(&mut rng);

        let mut table_512 = [0u8; 512];
        table_512[..256].copy_from_slice(&table_256);
        table_512[256..].copy_from_slice(&table_256);

        #[cfg(feature = "debug")]
        info!("Table finished in {:?}", start.elapsed());

        PerlinCPU { seed: table_512, scale, octaves, lacunarity, persistence }
    }

    pub fn from_point(&self, x: usize, y: usize) -> Vector {
        let index = self.seed[(self.seed[x % 256] as usize + y % 256) % 256] % VECTORS.len() as u8; 
        VECTORS[index as usize]
    }

    

    pub fn from_sample(&self, x: f32, y: f32) -> f32 {
        let x0 = floor(x) as usize;
        let y0 = floor(y) as usize;

        let g00 = self.from_point(x0, y0);
        let g10 = self.from_point(x0+1, y0);
        let g01 = self.from_point(x0, y0+1);
        let g11 = self.from_point(x0+1, y0+1);

        let dx = x - x0 as f32;
        let dy = y - y0 as f32;

        let offset_g00 = Vector::new(dx, dy);
        let offset_g10 = Vector::new(dx - 1., dy);
        let offset_g01 = Vector::new(dx, dy - 1.);
        let offset_g11 = Vector::new(dx - 1., dy - 1.);

        let n00 = g00.dot(offset_g00);
        let n10 = g10.dot(offset_g10);
        let n01 = g01.dot(offset_g01);
        let n11 = g11.dot(offset_g11);

        let fade_x = fade(dx);
        let fade_y = fade(dy);
        
        let nx0 = lerp(n00, n10, fade_x);
        let nx1 = lerp(n01, n11, fade_x);

        lerp(nx0, nx1, fade_y)
    }

    pub fn from_fractal(&self, x: f32, y: f32) -> f32 {
        let mut total = 0.;
        let mut frequency = 1.;
        let mut amplitude = 1.;
        let mut max_amplitude = 0.;

        for _ in 0..self.octaves {
            total += self.from_sample(x * frequency, y * frequency) * amplitude;
            max_amplitude += amplitude;
            frequency *= self.lacunarity;
            amplitude *= self.persistence;
        }

        total / max_amplitude
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
