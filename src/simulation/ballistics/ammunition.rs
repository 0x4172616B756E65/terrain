use bevy::{ecs::component::Component, math::Vec3};

use crate::simulation::world::{WorldState, GRAVITY};

#[derive(Component, Debug)]
pub struct Bullet {
    velocity: Vec3,
    spin: f32,
    air_density: f32,

    mass: f32,
    magnus: f32,
    diameter: f32,
    ballistic_coefficient: f32,
    drag_coefficient: f32,
    cross_section: f32,

    pub position: Vec3,
    distance_traveled: f32,
}

impl Bullet {
    pub fn new_nine_mm(direction: Vec3, muzzle_velocity: f32, barrel_spin: f32, world_state: &WorldState, position: Vec3) -> Self {
        Bullet {
            velocity: direction * muzzle_velocity,
            spin: barrel_spin,
            air_density: world_state.get_air_density(),

            mass: 0.115,
            magnus: 0.001,
            diameter: 0.009,
            ballistic_coefficient: 0.16,
            drag_coefficient: 0.3,
            cross_section: 0.0000636,

            position,
            distance_traveled: 0.,
        }
    }
}

pub trait Ballistics {
    fn instant_velocity(&self) -> &Vec3;
    fn step(&mut self, delta_time: f32) -> ();
} 

impl Ballistics for Bullet {
    fn instant_velocity(&self) -> &Vec3 { &self.velocity }
    fn step(&mut self, delta_time: f32) {
        let speed = self.velocity.length();
        let spin_axis = self.velocity.normalize();
        let drag_mag = 0.5 * self.air_density * (speed * speed) * self.drag_coefficient * self.cross_section;
        let drag_force = -drag_mag * self.velocity.normalize();

        let magnus_force = self.magnus * self.spin * self.velocity.cross(spin_axis);
        let gravity_force = Vec3::new(0.0, - GRAVITY * self.mass, 0.0);

        let acceleration = (drag_force + magnus_force + gravity_force) / self.mass;

        self.velocity += acceleration * delta_time;
        self.position += self.velocity * delta_time;
        self.distance_traveled += speed * delta_time;
    }
}
