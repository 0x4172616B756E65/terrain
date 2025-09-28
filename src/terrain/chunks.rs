use std::{collections::HashMap, sync::Arc};

use bevy::{asset::RenderAssetUsages, ecs::resource::Resource, math::Vec3, render::mesh::{Indices, Mesh, PrimitiveTopology}};
use bevy_rapier3d::{na::{DMatrix, Vector3}, prelude::Collider};

use crate::noise::perlin::Perlin;

#[derive(Resource, Debug)]
pub struct Chunkbase(HashMap<(i32, i32), Chunk>);

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
        let mut chunks = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                let mut chunk = Chunk::new(x, y, perlin);
                match normals {
                    true => chunk.generate_mesh_with_normals(),
                    false => chunk.generate_mesh()
                };
                chunks.insert((x, y), chunk);
            }
        }
        Chunkbase(chunks)
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


pub fn calculate_mesh(perlin: Perlin, width: u32, height: u32) -> (Vec<[f32; 3]>, Vec<u32>){
    let mut vertices: Vec<[f32; 3]> = Vec::with_capacity((width * height) as usize);
    let mut heightmap: Vec<f32> = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let fx = x as f32 * perlin.scale;
            let fy = y as f32 * perlin.scale;
            let z = (perlin.from_fractal(fx, fy) + 0.5).powi(4) * 10.;

            vertices.push([x as f32, z, y as f32]);
            heightmap.push(1_f32);
        }
    }

    let mut indices = Vec::with_capacity(((width - 1) * (height - 1) * 6) as usize);
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let i0 = x + y * width;
            let i1 = i0 + 1;
            let i2 = i0 + width;
            let i3 = i2 + 1;

            indices.push(i0);
            indices.push(i3);
            indices.push(i1);

            indices.push(i0);
            indices.push(i2);
            indices.push(i3);
        }
    }

    (vertices, indices)
}
