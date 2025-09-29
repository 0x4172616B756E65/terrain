use bevy::prelude::*;

use crate::noise::{perlin::Perlin, poisson_disc::PoissonDisc};

pub struct PropPlugin;

impl Plugin for PropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_props);
    }
}

fn load_props(mut commands: Commands, perlin: Res<Perlin>) {
    let mut _poisson = PoissonDisc::new(1., Vec2::new(32., 32.), 2);
    let points = vec![Vec2::new(1., 1.)];

    for point in points {
        let x = point.x;
        let y = perlin.from_fractal(x, point.y);
        let z = point.y;
        info!("Spawning tree at: {x}, {y}, {z}");
        commands.spawn((
            Transform::from_xyz(x, y, z),
    ));

    }
}
