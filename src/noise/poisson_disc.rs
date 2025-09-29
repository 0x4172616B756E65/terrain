use std::f32::consts::{PI, SQRT_2};

use bevy::math::Vec2;
use rand::Rng;

pub struct PoissonDisc {
    grid: Vec<Vec<Option<usize>>>,
    points: Vec<Vec2>,
    radius: f32,
    cell_size: f32,
    sample_size: Vec2,
    samples_try: i32,
}

impl PoissonDisc {
    pub fn new(radius: f32, sample_size: Vec2, samples_try: i32) -> Self {
        let cell_size = radius / SQRT_2;
        let grid_width = (sample_size.x / cell_size).ceil() as usize;
        let grid_height = (sample_size.y / cell_size).ceil() as usize;
        let grid = vec![vec![None; grid_height]; grid_width];
        let points: Vec<Vec2> = Vec::new();

        PoissonDisc { 
            grid,
            points,
            radius, 
            cell_size,
            sample_size, 
            samples_try 
        }
    }

    pub fn generate_points(&mut self) -> &Vec<Vec2> {
        let mut rng = rand::rng();
        let mut spawn_points: Vec<Vec2> = Vec::new();

        spawn_points.push(self.sample_size / 2.);
        while spawn_points.len() > 0 {
            let spawn_index = rng.random_range(0..spawn_points.len());
            let spawn_center: Vec2 = spawn_points[spawn_index];
            let mut candidate_accepted = false;

            for _ in 0..self.samples_try {
                let angle = rng.random_range(0_f32..1_f32) * PI * 2_f32;
                let direction: Vec2 = Vec2::new(angle.cos(), angle.sin());
                let candidate: Vec2 = spawn_center + direction * rng.random_range(self.radius..(2_f32 * self.radius));
                if self.is_valid(candidate) {
                    self.points.push(candidate); 
                    spawn_points.push(candidate);

                    let mut cell_x = (candidate.x / self.cell_size).floor() as usize;
                    let mut cell_y = (candidate.y / self.cell_size).floor() as usize;

                    cell_x = cell_x.min(self.grid.len() - 1);
                    cell_y = cell_y.min(self.grid[0].len() - 1);

                    self.grid[cell_x][cell_y] = Some(self.points.len() - 1); 

                    candidate_accepted = true;
                    break;
                }
            }

            if !candidate_accepted {
                spawn_points.remove(spawn_index);
            }
        }

        &self.points
    }

    fn is_valid(&self, candidate: Vec2) -> bool {
        if candidate.x >= 0. && candidate.x < self.sample_size.x &&
           candidate.y >= 0. && candidate.y < self.sample_size.y {
            let cell_x = (candidate.x / self.cell_size) as usize;
            let cell_y = (candidate.y / self.cell_size) as usize;

            let search_start_x = cell_x.saturating_sub(2);
            let search_start_y = cell_y.saturating_sub(2);
            let search_end_x = (self.grid.len() - 1).min(cell_x + 2);
            let search_end_y = (self.grid[0].len() - 1).min(cell_y + 2);


            for x in search_start_x..=search_end_x {
                for y in search_start_y..=search_end_y {
                    if let Some(point_index) = self.grid[x][y] {
                        let distance_sq = (candidate - self.points[point_index]).length_squared();
                        if distance_sq < self.radius * self.radius { 
                            return false; 
                        }
                    }
                }
            }
        }
        true
    }
}
