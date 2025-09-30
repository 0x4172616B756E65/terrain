#[cfg(feature = "debug")]
use std::time::Instant;
use std::{collections::HashMap};

use bevy::{asset::RenderAssetUsages, ecs::{entity::Entity, resource::Resource}, render::mesh::{Indices, Mesh, PrimitiveTopology}, tasks::block_on};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
#[cfg(feature = "debug")]
use tracing::info;

use crate::noise::{perlin::Perlin, perlin_cpu::PerlinCPU};

#[derive(Resource, Debug)]
pub struct Chunkbase(HashMap<(i32, i32), Chunk>);

#[derive(Resource, Default)]
pub struct RenderedChunks(pub HashMap<(i32, i32), Entity>);

#[derive(Resource)]
pub struct RenderDistance(pub i32);


impl Chunkbase { 
    pub fn new(height: i32, width: i32, perlin: &Perlin) -> Self { 
        let mut chunks = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                chunks.insert((x, y), Chunk::new(x, y, perlin));
            }
        }
        Chunkbase(chunks)
    }

    pub fn new_with_mesh(height: i32, width: i32, perlin: &Perlin, normals: bool) -> Self { 
        let chunks: HashMap<(i32, i32), Chunk> = (0..height)
            .into_par_iter()
            .flat_map(|y| (0..width).map(move |x| (x, y)).par_bridge())
            .map(|(x, y)| {
                let mut chunk = Chunk::new(x, y, perlin);
                match normals {
                    true => chunk.generate_mesh_with_normals(),
                    false => chunk.generate_mesh()
                };
                ((x, y), chunk)
            }).collect();
             
        Chunkbase(chunks)
    }

    pub fn load_chunk(&self, coordinates: &(i32, i32)) -> Option<&Chunk> {
        self.0.get(coordinates)
    }

    pub fn load_chunks(&self, cx: i32, cy: i32, radius: i32) -> Vec<&Chunk> {
        let mut chunks = Vec::with_capacity((radius * 2 + 1).pow(2) as usize);
        let radius_sq = radius * radius;

        for y in -radius..=radius {
            let y_sq = y * y;
            for x in -radius..=radius {
                if x * x + y_sq <= radius_sq {
                    let chunk_coords = (cx.wrapping_add(x), cy.wrapping_add(y));
                    if let Some(chunk) = self.0.get(&chunk_coords) {
                        chunks.push(chunk);
                    }
                }
            }
        }

        /* WIP parallelization of chunk loading
        let chunks: Vec<&Chunk> = (-radius..=radius)
            .into_par_iter()
            .flat_map(|y| (-radius..=radius).filter_map(move |x| { if x * x + y * y <= radius * radius {(x, y)}}).par_bridge())
            .map(|(x, y)| {
                let chunk_coords = (cx.wrapping_add(x), cy.wrapping_add(y));
                if let Some(chunk) = self.0.get(&chunk_coords) { chunk }
            }).collect();*/

        chunks
    }

    pub fn load_chunks_from_map(&self, map: Vec<(i32, i32)>) -> Vec<&Chunk> {
        let mut chunks = Vec::with_capacity(map.len());
        for coords in map { if let Some(chunk) = self.0.get(&coords) { chunks.push(chunk); } }
        chunks
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub  vertex_buffer: Vec<[f32; 3]>,
    pub  index_buffer: Vec<u32>,
    mesh: Option<Mesh>,
}   

impl Chunk {
    pub fn new(chunk_x: i32, chunk_y: i32, perlin: &Perlin) -> Self {
        #[cfg(feature = "debug")]
        let start = Instant::now();
        let heightmap = block_on(perlin.compute_from_fractal((32/8, 32/8, 1))).unwrap();

        #[cfg(feature = "debug")]
        info!("Thread locked for: {:?}", start.elapsed());

        let mut vertex_buffer: Vec<[f32; 3]> = Vec::with_capacity(1024);  
        let mut index_buffer: Vec<u32> = Vec::with_capacity(5766);

        for y in 0..32 {
            for x in 0..32 {
                vertex_buffer.push([(x + chunk_x * 32) as f32, heightmap[(x + (32*y)) as usize], (y + chunk_y * 32) as f32]);
            }
        }

        for y in 0..31 {
            for x in 0..31 {
                let i0 = x + y * 32;
                let i1 = i0 + 1;
                let i2 = i0 + 32;
                let i3 = i2 + 1;

                index_buffer.push(i0);
                index_buffer.push(i3);
                index_buffer.push(i1);

                index_buffer.push(i0);
                index_buffer.push(i2);
                index_buffer.push(i3);
            }
        }

        Chunk { vertex_buffer, index_buffer, mesh: None }

    }
    pub fn new_cpu(chunk_x: i32, chunk_y: i32, perlin: &PerlinCPU) -> Self {
        let mut vertex_buffer: Vec<[f32; 3]> = Vec::with_capacity(1024);  
        let mut index_buffer: Vec<u32> = Vec::with_capacity(5766);

        for y in 0..32 {
            for x in 0..32 {
                let fx = (x + chunk_x * 32) as f32 * perlin.scale;
                let fy = (y + chunk_y * 32) as f32 * perlin.scale;
                let z = (perlin.from_fractal(fx, fy) + 0.5).powi(4) * 10.;

                vertex_buffer.push([(x + chunk_x * 32) as f32, z, (y + chunk_y * 32) as f32]);
            }
        }

        for y in 0..31 {
            for x in 0..31 {
                let i0 = x + y * 32;
                let i1 = i0 + 1;
                let i2 = i0 + 32;
                let i3 = i2 + 1;

                index_buffer.push(i0);
                index_buffer.push(i3);
                index_buffer.push(i1);

                index_buffer.push(i0);
                index_buffer.push(i2);
                index_buffer.push(i3);
            }
        }

        Chunk { vertex_buffer, index_buffer, mesh: None }
    }

    pub fn generate_mesh(&mut self) -> &Mesh {
        if self.mesh.is_none() {
            self.mesh = Some(Mesh::new(PrimitiveTopology::TriangleList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.vertex_buffer.clone())
                .with_inserted_indices(Indices::U32(self.index_buffer.clone()))
            );
        }
        self.mesh.as_ref().unwrap()
    }

    pub fn generate_mesh_with_normals(&mut self) -> &Mesh {
        if self.mesh.is_none() {
            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.vertex_buffer.clone())
                .with_inserted_indices(Indices::U32(self.index_buffer.clone()));
            mesh.compute_smooth_normals();
            self.mesh = Some(mesh);
        }

        self.mesh.as_ref().unwrap()
    }


    pub fn get_mesh(&self) -> &Option<Mesh> {
        &self.mesh 
    }
}
