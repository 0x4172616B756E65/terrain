use std::f32::consts::PI;
use bevy::math::Vec2;
use rand::prelude::*;

pub struct PoissonDisc {
    grid: Vec<Vec<Option<usize>>>,
    points: Vec<Vec2>,
    radius: f32,
    cell_size: f32,
    sample_size: Vec2,
    samples_try: usize,
}

impl PoissonDisc {
    pub fn new(radius: f32, sample_size: Vec2, samples_try: usize) -> Self {
        let cell_size = radius / 2f32.sqrt();
        let grid_width = (sample_size.x / cell_size).ceil().max(1.0) as usize;
        let grid_height = (sample_size.y / cell_size).ceil().max(1.0) as usize;
        let grid = vec![vec![None; grid_height]; grid_width];

        PoissonDisc {
            grid,
            points: Vec::new(),
            radius,
            cell_size,
            sample_size,
            samples_try,
        }
    }

    pub fn generate_points(&mut self) -> &Vec<Vec2> {
        let mut rng = rand::rng();

        for col in &mut self.grid {
            for cell in col.iter_mut() {
                *cell = None;
            }
        }
        self.points.clear();

        let initial = self.sample_size / 2.0;
        self.points.push(initial);

        let w = self.grid.len();
        let h = self.grid[0].len();
        let mut ix = (initial.x / self.cell_size).floor() as isize;
        let mut iy = (initial.y / self.cell_size).floor() as isize;
        ix = ix.clamp(0, (w as isize) - 1);
        iy = iy.clamp(0, (h as isize) - 1);
        self.grid[ix as usize][iy as usize] = Some(0);

        let mut spawn_points = Vec::new();
        spawn_points.push(initial);

        while !spawn_points.is_empty() {
            let spawn_index = rng.random_range(0..spawn_points.len());
            let spawn_center = spawn_points[spawn_index];
            let mut accepted = false;

            for _ in 0..self.samples_try {
                let angle = rng.random_range(0.0..(2.0 * PI));
                let r = rng.random_range(self.radius..(2.0 * self.radius));
                let candidate = spawn_center + Vec2::new(angle.cos(), angle.sin()) * r;

                if self.is_valid(candidate) {
                    let new_index = self.points.len();
                    self.points.push(candidate);
                    spawn_points.push(candidate);

                    let mut cell_x = (candidate.x / self.cell_size).floor() as isize;
                    let mut cell_y = (candidate.y / self.cell_size).floor() as isize;
                    cell_x = cell_x.clamp(0, (w as isize) - 1);
                    cell_y = cell_y.clamp(0, (h as isize) - 1);

                    self.grid[cell_x as usize][cell_y as usize] = Some(new_index);
                    accepted = true;
                    break;
                }
            }

            if !accepted {
                spawn_points.swap_remove(spawn_index);
            }
        }

        &self.points
    }

    fn is_valid(&self, candidate: Vec2) -> bool {
        if !(candidate.x >= 0.0
            && candidate.x < self.sample_size.x
            && candidate.y >= 0.0
            && candidate.y < self.sample_size.y)
        {
            return false;
        }

        let cell_x = (candidate.x / self.cell_size).floor() as isize;
        let cell_y = (candidate.y / self.cell_size).floor() as isize;

        let w = self.grid.len() as isize;
        let h = self.grid[0].len() as isize;

        let start_x = (cell_x - 2).max(0) as usize;
        let start_y = (cell_y - 2).max(0) as usize;
        let end_x = (cell_x + 2).min(w - 1) as usize;
        let end_y = (cell_y + 2).min(h - 1) as usize;

        let rr = self.radius * self.radius;
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                if let Some(pi) = self.grid[x][y] {
                    let dist2 = (candidate - self.points[pi]).length_squared();
                    if dist2 < rr {
                        return false;
                    }
                }
            }
        }

        true
    }
}
