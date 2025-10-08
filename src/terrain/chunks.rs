#[cfg(feature = "debug")]
use std::time::Instant;
use std::{collections::HashMap};

use bevy::prelude::*;
use bevy::{asset::RenderAssetUsages, ecs::{bundle::Bundle, entity::Entity, resource::Resource}, render::mesh::{Indices, Mesh, PrimitiveTopology}, tasks::block_on};
use bevy_rapier3d::prelude::Collider;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
#[cfg(feature = "debug")]
use tracing::info;

use crate::noise::{perlin::Perlin, perlin_cpu::PerlinCPU};

pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;
pub const CHUNK_HEIGHT: usize = 128;
pub const CHUNK_WIDTH: usize = 128;

#[derive(Resource, Debug)]
pub struct Chunkbase(HashMap<(i32, i32), Chunk>);

#[derive(Resource, Default)]
pub struct RenderedChunks(pub HashMap<(i32, i32), Entity>);

#[derive(Resource)]
pub struct RenderDistance(pub i32);


impl Chunkbase { 
    pub fn new(perlin: &Perlin) -> Self { 
        let workgroup_size_x = 8;
        let workgroup_size_y = 8;

        let total_width  = MAP_WIDTH * CHUNK_WIDTH;
        let total_height = MAP_HEIGHT * CHUNK_HEIGHT;

        let workgroups_x = (total_width  + workgroup_size_x - 1) / workgroup_size_x;
        let workgroups_y = (total_height + workgroup_size_y - 1) / workgroup_size_y;

        #[cfg(feature = "debug")]
        let start = Instant::now();
        let heightmap = block_on(perlin.compute_from_fractal((workgroups_x as u32, workgroups_y as u32, 1))).unwrap();
        #[cfg(feature = "debug")]
        info!("Thread locked for: {:?}", start.elapsed());
        let chunkmap: &[[[f32; CHUNK_WIDTH]; CHUNK_HEIGHT]; 16384] =  unsafe { &*(heightmap.as_ptr() as *const [[[f32; CHUNK_WIDTH]; CHUNK_HEIGHT]; 16384]) };

        let chunks: HashMap<(i32, i32), Chunk> = (0..MAP_HEIGHT)
            .into_par_iter()
            .flat_map_iter(|chunk_y| (0..MAP_WIDTH).map(move |chunk_x| (chunk_x, chunk_y)))
            .map(|(x, y)| {
                let slice: &[[f32; CHUNK_WIDTH]; CHUNK_HEIGHT] = &chunkmap[x + y * MAP_WIDTH];

                let mut halo: [f32; CHUNK_WIDTH + CHUNK_HEIGHT + 1] = [0.0; CHUNK_WIDTH + CHUNK_HEIGHT + 1];
                if x < MAP_WIDTH - 1 && y < MAP_HEIGHT - 1 {
                    //Init a halo array that holds
                    //[x; 32] for horizontal haloing
                    //[y; 32] for vertical haloing
                    //[z] for bottom-right corner haloing

                    //When y = 33
                    //[x; 32] = 
                    halo[0..CHUNK_WIDTH].copy_from_slice(&chunkmap[x + (y + 1) * MAP_WIDTH][0]);

                    // When x = 33
                    //[y; 32] =
                    for cy in 0..CHUNK_HEIGHT {
                        halo[CHUNK_WIDTH + cy] = chunkmap[(x + 1) + y * MAP_WIDTH][cy][0];
                    }

                    //When x, y = 33
                    //[z] = curr x + 1, curr y + 1
                    halo[CHUNK_HEIGHT + CHUNK_WIDTH] = chunkmap[(x + 1) + ((y + 1) * MAP_WIDTH)][0][0]; 
                }

                let mut chunk_data = ChunkData::new(slice, &halo);
                let mesh = chunk_data.into_mesh_with_normals();

                let flat_slice = slice.iter().flat_map(|row| row.iter().copied()).collect();
                let collider = Collider::heightfield(flat_slice, CHUNK_WIDTH, CHUNK_HEIGHT, Vec3::ONE);
                let chunk = Chunk { 
                    transform: Transform::from_xyz((x * CHUNK_WIDTH) as f32, 0., (y * CHUNK_HEIGHT) as f32), 
                    collider: collider,
                    mesh: mesh,
                };

                ((x as i32, y as i32), chunk)
            }).collect();
             
        Chunkbase(chunks)
    }

    pub fn get_chunk(&self, coordinates: &(i32, i32)) -> Option<&Chunk> {
        self.0.get(coordinates)
    }
}

pub struct ChunkData {
    pub  vertex_buffer: Vec<[f32; 3]>,
    pub  index_buffer: Vec<u32>,
}

#[derive(/*Bundle,*/ Debug)]
pub struct Chunk {
    pub transform: Transform,
    pub collider: Collider,
    pub mesh: Mesh,
}

impl Chunk {
    //pub fn new(x: usize, y: usize, mesh: Mesh) -> Self { Chunk { x, y, mesh } }
    //pub fn get_mesh(&self) -> &Mesh { &self.mesh }
}

impl ChunkData {
    pub fn new(heightmap: &[[f32; CHUNK_WIDTH]; CHUNK_HEIGHT], halo: &[f32; CHUNK_HEIGHT + CHUNK_WIDTH + 1]) -> Self {
        let vertex_buffer: Vec<[f32; 3]> = (0..=CHUNK_HEIGHT)
            .into_par_iter()
            .flat_map_iter(|y| (0..=CHUNK_WIDTH).map(move |x| (x, y)))
            .map(|(x, y)| {
                 [
                    x as f32,
                    match(y, x) {
                        (CHUNK_HEIGHT, CHUNK_WIDTH) => (halo[CHUNK_HEIGHT + CHUNK_WIDTH] + 1.0).powi(4),
                        (_, CHUNK_WIDTH) => (halo[y + CHUNK_WIDTH] + 1.0).powi(4),
                        (CHUNK_HEIGHT, _) => (halo[x] + 1.0).powi(4),
                        _ => (heightmap[y][x] + 1.0).powi(4)
                    },
                    y as f32
                ]
            }).collect();

        let index_buffer: Vec<u32> = (0..CHUNK_HEIGHT)
            .into_par_iter()
            .flat_map_iter(|y| { 
                (0..CHUNK_WIDTH).flat_map(move |x| {
                    let stride = (CHUNK_WIDTH + 1) as u32;
                    let i0 = x as u32 + y as u32 * stride;
                    let i1 = i0 + 1;
                    let i2 = i0 + stride;
                    let i3 = i2 + 1;

                    [i0, i3, i1, i0, i2, i3]
                })
            }).collect();

        ChunkData { vertex_buffer, index_buffer }
    }

    pub fn into_mesh(&mut self) -> Mesh {
        Mesh::new(PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.vertex_buffer.clone())
            .with_inserted_indices(Indices::U32(self.index_buffer.clone()))
    }

    pub fn into_mesh_with_normals(&mut self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.vertex_buffer.clone())
            .with_inserted_indices(Indices::U32(self.index_buffer.clone()));
        mesh.compute_smooth_normals();
        mesh
    }
}
