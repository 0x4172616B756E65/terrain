#[cfg(feature = "debug")]
use std::time::Instant;
use std::{collections::HashMap};

use bevy::{asset::RenderAssetUsages, ecs::{entity::Entity, resource::Resource}, render::mesh::{Indices, Mesh, PrimitiveTopology}, tasks::block_on};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
#[cfg(feature = "debug")]
use tracing::info;

use crate::noise::{perlin::Perlin, perlin_cpu::PerlinCPU};

pub const MAP_WIDTH: u32 = 128;
pub const MAP_HEIGHT: u32 = 128;
pub const CHUNK_HEIGHT: u32 = 32;
pub const CHUNK_WIDTH: u32 = 32;
pub const CHUNK_AREA: u32 = 1024;

#[derive(Resource, Debug)]
pub struct Chunkbase(HashMap<(i32, i32), Chunk>);

#[derive(Resource, Default)]
pub struct RenderedChunks(pub HashMap<(i32, i32), Entity>);

#[derive(Resource)]
pub struct RenderDistance(pub i32);


impl Chunkbase { 
    /*
    pub fn new(perlin: &Perlin) -> Self { 
        let mut chunks = HashMap::new();
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                chunks.insert((x, y), Chunk::new(x, y, perlin));
            }
        }
        Chunkbase(chunks)
    }*/

    pub fn new_with_mesh(perlin: &Perlin, normals: bool) -> Self { 
        let workgroup_size_x = 8;
        let workgroup_size_y = 8;

        let total_width  = MAP_WIDTH * CHUNK_WIDTH;
        let total_height = MAP_HEIGHT * CHUNK_HEIGHT;

        let workgroups_x = (total_width  + workgroup_size_x - 1) / workgroup_size_x;
        let workgroups_y = (total_height + workgroup_size_y - 1) / workgroup_size_y;

        #[cfg(feature = "debug")]
        let start = Instant::now();
        let heightmap = block_on(perlin.compute_from_fractal((workgroups_x, workgroups_y, 1))).unwrap();
        #[cfg(feature = "debug")]
        info!("Thread locked for: {:?}", start.elapsed());

       
        #[cfg(feature = "debug")]
        info!("Heightmap length: {:?}", heightmap.len());

        let row_width_in_elements = MAP_WIDTH * CHUNK_AREA;
        let chunks: HashMap<(i32, i32), Chunk> = (0..MAP_HEIGHT)
            .into_par_iter()
            .flat_map_iter(|chunk_y| (0..MAP_WIDTH).map(move |chunk_x| (chunk_x, chunk_y)))
            .map(|(chunk_x, chunk_y)| {
                let start = (chunk_y * row_width_in_elements + chunk_x * CHUNK_AREA) as usize;
                let end = start + CHUNK_AREA as usize;

                let slice = &heightmap[start..end];

                let mut chunk = Chunk::new(chunk_x, chunk_y, slice);

                if normals { chunk.generate_mesh_with_normals();} 
                else { chunk.generate_mesh(); }              
                
                ((chunk_x as i32, chunk_y as i32), chunk)
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
    pub fn new(chunk_x: u32, chunk_y: u32, heightmap: &[f32]) -> Self {

        let mut vertex_buffer: Vec<[f32; 3]> = Vec::with_capacity(1024);  
        let mut index_buffer: Vec<u32> = Vec::with_capacity(5766);

        for y in 0..CHUNK_HEIGHT {
            #[cfg(feature = "debug")]
            let mut heightrow = Vec::new();
            for x in 0..CHUNK_WIDTH {
                vertex_buffer.push([
                    (x + chunk_x * CHUNK_WIDTH) as f32,
                    (heightmap[(y + x*CHUNK_WIDTH) as usize] + 1.0).powi(4),
                    (y + chunk_y * CHUNK_HEIGHT) as f32,
                ]); 
                #[cfg(feature = "debug")]
                heightrow.push((heightmap[(y + x*CHUNK_WIDTH) as usize] + 1.0).powi(4));
            }
            #[cfg(feature = "debug")]
            info!("Heighrow: {:?}", heightrow);
            #[cfg(feature = "debug")]
            heightrow.clear();
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
