use bevy::{color::palettes::css::{ALICE_BLUE, WHITE}, pbr::light_consts::lux::{FULL_DAYLIGHT, FULL_MOON_NIGHT}, prelude::*, reflect::DynamicTypePath};

use crate::simulation::world::WorldState;

pub struct DaylightCyclePlugin;

impl Plugin for DaylightCyclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_sun_and_moon)
            .add_systems(Update, cycle_daylight);
    }
}

#[derive(Bundle)]
pub struct Sun {
    pub transform: Transform,
    pub light: DirectionalLight,
    sun_comp: SunComp
}

#[derive(Component)]
pub struct SunComp;

impl Sun {
    pub fn new() -> Self {
        Sun { 
            transform: Transform::from_xyz(0., 0., 0.),
            light: DirectionalLight { 
                color: WHITE.into(), 
                illuminance: FULL_DAYLIGHT, 
                ..Default::default()
            },
            sun_comp: SunComp
        }
    }
}

#[derive(Bundle)]
pub struct Moon {
    pub transform: Transform,
    pub light: DirectionalLight,
    moon_comp: MoonComp
}

#[derive(Component)]
pub struct MoonComp;

impl Moon {
    pub fn new() -> Self {
        Moon {
            transform: Transform::from_xyz(0., 0., 0.),
            light: DirectionalLight {
                color: ALICE_BLUE.into(),
                illuminance: FULL_MOON_NIGHT,
                ..Default::default()
            },
            moon_comp: MoonComp
        }
    }
}

fn spawn_sun_and_moon(mut commands: Commands) {
    //commands.spawn(Sun::new()); 
    commands.spawn(Moon::new()); 
}

fn cycle_daylight(
    world_state: ResMut<WorldState>, 
    mut celestial_query: ParamSet<(
        Query<(&mut DirectionalLight, &mut Transform), With<SunComp>>,
        Query<(&mut DirectionalLight, &mut Transform), With<MoonComp>>,
    )>,
) {
    let hour = world_state.get_hour();
    let th = ((*hour + 5.0) * 15.0).to_radians();
    let qrot = Quat::from_rotation_x(th);
    let color = kelvin_to_rgb(bell_kelvin(hour)); 

    let mut sun = celestial_query.p0();
    let (mut sun_light, mut sun_rotation) = sun.single_mut().unwrap();
        sun_light.color = Color::srgb_u8(color.0, color.1, color.2);
        sun_rotation.rotation = qrot;

    let mut moon = celestial_query.p1();
    let (_, mut moon_rotation) = moon.single_mut().unwrap();
        moon_rotation.rotation = -qrot;
}

fn bell_kelvin(x: &f32) -> f32 {
    let d = 1600.0;
    let a = 4400.0;
    let b = 0.0;
    let c = 2.6;

    d + a * (-((x - (b + 12.))*(x - (b + 12.))) / (2.0 * c*c)).exp()
}

fn kelvin_to_rgb(k: f32) -> (u8, u8, u8) {
    let temp = k.clamp(1000., 40000.) / 100.0;

    let mut red: f32;
    let mut green: f32;
    let mut blue: f32;

    if temp <= 66.0 {
        red = 255.0;
    } else {
        red = temp - 60.0;
        red = 329.698727446 * red.powf(-0.1332047592);
        red = red.clamp(0.0, 255.0);
    }

    if temp <= 66.0 {
        green = 99.4708025861 * temp.ln() - 161.1195681661;
    } else {
        green = temp - 60.0;
        green = 288.1221695283 * green.powf(-0.0755148492);
    }
    green = green.clamp(0.0, 255.0);

    if temp >= 66.0 {
        blue = 255.0;
    } else if temp <= 19.0 {
        blue = 0.0;
    } else {
        blue = temp - 10.0;
        blue = 138.5177312231 * blue.ln() - 305.0447927307;
    }
    blue = blue.clamp(0.0, 255.0);

    (red as u8, green as u8, blue as u8)
}

