use bevy::{ecs::resource::Resource, math::Vec2};
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, wgt::PollType, BindGroup, Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, DeviceDescriptor, MapMode, PipelineCompilationOptions, Queue, WasmNotSend};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use crate::terrain::chunks::{CHUNK_HEIGHT, CHUNK_WIDTH, MAP_HEIGHT, MAP_WIDTH};

#[derive(Debug, Clone, Resource)]
pub struct Perlin {
    device: Device,
    queue: Queue,

    size_buffer: Buffer,
    seed_buffer: Buffer,
    vector_buffer: Buffer,
    output_buffer: Buffer,
    //readback_buffer: Buffer,

    compute_pipeline: ComputePipeline,
    bind_group: BindGroup
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Size {
    width: u32,
    height: u32,
}

impl Perlin {
    pub async fn new(seed: u64) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            }).await.unwrap();

        let (device, queue) = adapter.request_device(&DeviceDescriptor::default()).await.unwrap();

        let mut table_256: [u32; 256] = (0..=255).collect::<Vec<u32>>().try_into().unwrap();
        let mut rng = StdRng::seed_from_u64(seed);
        table_256.shuffle(&mut rng);

        let table_512: [u32; 512] = {
            let mut arr = [0u32; 512];
            arr[..256].copy_from_slice(&table_256);
            arr[256..].copy_from_slice(&table_256);
            arr
        };

        let vectors_data: [Vec2; 8] = [
            Vec2::new(1.0,0.0), Vec2::new(-1.0,0.0),
            Vec2::new(0.0,1.0), Vec2::new(0.0,-1.0),
            Vec2::new(1.0,1.0), Vec2::new(-1.0,1.0),
            Vec2::new(1.0,-1.0), Vec2::new(-1.0,-1.0),
        ];

        let size_data = Size { width: MAP_WIDTH, height: MAP_HEIGHT };
        let vertex_count = (MAP_HEIGHT * MAP_WIDTH) as u64;
        let buffer_size = vertex_count * CHUNK_WIDTH as u64 * CHUNK_HEIGHT as u64 * std::mem::size_of::<f32>() as u64;

         let size_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("SizeBuffer"),
            contents: bytemuck::bytes_of(&size_data),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        }); 

        let seed_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("SeedBuffer"),
            contents: bytemuck::cast_slice(&table_512),
            usage: BufferUsages::STORAGE,
        });

        let vector_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("VectorBuffer"),
            contents: bytemuck::cast_slice(&vectors_data),
            usage: BufferUsages::STORAGE,
        });

        let output_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("OutputBuffer"),
            size: buffer_size, 
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        /*        
        let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ReadbackBuffer"),
            size: 32*32*std::mem::size_of::<f32>() as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        */

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("from_point"),
            source: wgpu::ShaderSource::Wgsl(include_str!("perlin.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout"),
            entries: &[
               wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("BindGroup"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: size_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: seed_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: vector_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: output_buffer.as_entire_binding() },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("PipelineLayout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("ComputePipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions::default(),
            cache: None
        });

        Ok(Self {
            device,
            queue,

            size_buffer,
            seed_buffer,
            vector_buffer,
            output_buffer,
            //readback_buffer,
            compute_pipeline,
            bind_group,
        })
    }

    //pub async fn compute_from_sample(&self, workgroups: (u32, u32, u32)) -> anyhow::Result<&[f32]> { self.dispatch(workgroups).await }
    pub async fn compute_from_fractal(&self, workgroups: (u32, u32, u32)) -> anyhow::Result<Vec<f32>> { self.dispatch(workgroups).await }

    async fn dispatch(&self, workgroups: (u32, u32, u32)) -> anyhow::Result<Vec<f32>> {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("ComputeEncoder") });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.bind_group, &[]);
            compute_pass.dispatch_workgroups(workgroups.0, workgroups.1, workgroups.2);
        }

        let vertex_count = (MAP_HEIGHT * MAP_WIDTH) as u64;
        let buffer_size = vertex_count * CHUNK_WIDTH as u64 * CHUNK_HEIGHT as u64 * std::mem::size_of::<f32>() as u64;

        let readback_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("ReadbackBuffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        encoder.copy_buffer_to_buffer(
            &self.output_buffer, 0,
            &readback_buffer, 0,
            buffer_size 
        );

        self.queue.submit(Some(encoder.finish()));

        let buffer_slice = readback_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        self.device.poll(PollType::Wait).unwrap();
        receiver.receive().await.unwrap()?;

        let data = buffer_slice.get_mapped_range();
        let result = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        readback_buffer.unmap();

        Ok(result)
    }
}
