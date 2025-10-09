use std::{collections::HashMap};

use bevy::prelude::*;
use bevy::{asset::RenderAssetUsages, ecs::{entity::Entity, resource::Resource}, render::mesh::{Indices, Mesh, PrimitiveTopology}, tasks::block_on};
use bevy_rapier3d::prelude::Collider;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::noise::{perlin::Perlin};

pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;
pub const CHUNK_HEIGHT: usize = 128;
pub const CHUNK_WIDTH: usize = 128;

#[derive(Resource, Debug)]
pub struct Chunkbase(HashMap<(i32, i32), Chunk>);

#[derive(Resource, Default)]
pub struct RenderedChunks(pub HashMap<(i32, i32), Entity>);

#[derive(Resource)]
pub struct RenderDistance(pub u32);


impl Chunkbase { 
    pub fn new(perlin: &Perlin) -> Self { 
        let workgroup_size_x = 8;
        let workgroup_size_y = 8;

        let total_width  = MAP_WIDTH * CHUNK_WIDTH;
        let total_height = MAP_HEIGHT * CHUNK_HEIGHT;

        let workgroups_x = (total_width  + workgroup_size_x - 1) / workgroup_size_x;
        let workgroups_y = (total_height + workgroup_size_y - 1) / workgroup_size_y;

        let heightmap = block_on(perlin.compute_from_fractal((workgroups_x as u32, workgroups_y as u32, 1))).unwrap();
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

                let mut chunk_data = ChunkData::new(slice, &halo, 1);
                let mut chunk_data_2 = ChunkData::new(slice, &halo, 2);
                let mut chunk_data_4 = ChunkData::new(slice, &halo, 4);

                let mesh = chunk_data.into_mesh_with_normals();
                let mesh_2 = chunk_data_2.into_mesh_with_normals();
                let mesh_4 = chunk_data_4.into_mesh_with_normals();

                let flat_slice = slice.iter().flat_map(|row| row.iter().copied()).collect();
                let collider = generate_heightfield(flat_slice, 16);
                let chunk = Chunk { 
                    transform: Transform::from_xyz((x * CHUNK_WIDTH) as f32, 0., (y * CHUNK_HEIGHT) as f32), 
                    collider: collider,
                    mesh,
                    mesh_2,
                    mesh_4
                };

                ((x as i32, y as i32), chunk)
            }).collect();
             
        Chunkbase(chunks)
    }

    pub fn get_chunk(&self, coordinates: &(i32, i32)) -> Option<&Chunk> {
        self.0.get(coordinates)
    }

}

fn generate_heightfield(heightmap: Vec<f32>, lod: usize) -> Collider {
    let mut heightfield = Vec::with_capacity(CHUNK_HEIGHT / lod + CHUNK_WIDTH / lod);
    for y in (0..CHUNK_HEIGHT).step_by(lod) {
        for x in (0..CHUNK_WIDTH).step_by(lod) {
            heightfield.push((heightmap[y * CHUNK_WIDTH + x] + 1.0).powi(4) * 30.0);
        }
    }

    Collider::heightfield(heightfield, CHUNK_HEIGHT / lod, CHUNK_WIDTH/ lod, Vec3::new(CHUNK_HEIGHT as f32, 1.0, CHUNK_WIDTH as f32))
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
    pub mesh_2: Mesh,
    pub mesh_4: Mesh,
}

impl Chunk {
    //pub fn new(x: usize, y: usize, mesh: Mesh) -> Self { Chunk { x, y, mesh } }
    //pub fn get_mesh(&self) -> &Mesh { &self.mesh }
}

impl ChunkData {
    pub fn new(heightmap: &[[f32; CHUNK_WIDTH]; CHUNK_HEIGHT], halo: &[f32; CHUNK_HEIGHT + CHUNK_WIDTH + 1], lod: usize) -> Self {
        match lod {
            1 => {
                let vertex_buffer: Vec<[f32; 3]> = (0..=CHUNK_HEIGHT)
                    .into_par_iter()
                    .flat_map_iter(|y| (0..=CHUNK_WIDTH).map(move |x| (x, y)))
                    .map(|(x, y)| {
                        [
                            x as f32,
                            match(y, x) {
                                (CHUNK_HEIGHT, CHUNK_WIDTH) => (halo[CHUNK_HEIGHT + CHUNK_WIDTH] + 1.0).powi(4) * 30.,
                                (_, CHUNK_WIDTH) => (halo[y + CHUNK_WIDTH] + 1.0).powi(4) * 30.,
                                (CHUNK_HEIGHT, _) => (halo[x] + 1.0).powi(4) * 30.,
                                _ => (heightmap[y][x] + 1.0).powi(4) * 30.,
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
            },
            lod => {
                let mut vertex_buffer: Vec<[f32; 3]> = Vec::new();
                let mut index_buffer: Vec<u32> = Vec::new();

                let lod_height = CHUNK_HEIGHT / lod;
                let lod_width = CHUNK_WIDTH / lod;

                for y in 0..=lod_height {
                    for x in 0..=lod_width {
                        vertex_buffer.push([
                            (x * lod) as f32,
                            match(y, x) {
                                _ if y == lod_height && x == lod_width => (halo[CHUNK_HEIGHT + CHUNK_WIDTH] + 1.0).powi(4) * 30.,
                                _ if x == lod_width => (halo[y * lod + CHUNK_WIDTH] + 1.0).powi(4) * 30.,
                                _ if y == lod_height => (halo[x * lod] + 1.0).powi(4) * 30.,
                                _ => (heightmap[y.saturating_sub(1) * lod][x.saturating_sub(1) * lod] + 1.0).powi(4) * 30.
                            },
                            (y * lod) as f32
                        ]);
                    }
                }

                //info!("Vertex buffer length: {}", vertex_buffer.len());
                //std::thread::sleep(Duration::new(4, 0));

                for y in 0..lod_height as u32 {
                    for x in 0..lod_width as u32 {
                        let stride = lod_width as u32;
                        let i0 = x + y * stride;
                        let i1 = i0 + 1;
                        let i2 = i0 + stride;
                        let i3 = i2 + 1;


                        index_buffer.push(i0);
                        index_buffer.push(i3);
                        index_buffer.push(i1);

                        index_buffer.push(i0);
                        index_buffer.push(i2);
                        index_buffer.push(i3);
                    }
                }

                //let expected_len = lod_height * lod_width * 6;
                //info!("Index buffer length: {}, Expected length: {}", index_buffer.len(), expected_len);
                //std::thread::sleep(Duration::new(4, 0));

                ChunkData { vertex_buffer, index_buffer, }
            }
        }
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
