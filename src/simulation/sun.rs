use std::time::SystemTime;

use bevy::{color::palettes::css::WHITE, pbr::light_consts::lux::FULL_DAYLIGHT, prelude::*};

pub struct DaylightCyclePlugin;

impl Plugin for DaylightCyclePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Timer::default())
            .add_systems(Startup, spawn_sun)
            .add_systems(Update, cycle_daylight);
    }
}

#[derive(Debug, Bundle)]
pub struct Sun {
    pub transform: Transform,
    pub light: DirectionalLight
}

impl Sun {
    pub fn new(transform: Transform) -> Self {
        Sun { 
            transform: transform, 
            light: DirectionalLight { 
                color: WHITE.into(), 
                illuminance: FULL_DAYLIGHT, 
                ..Default::default()
            }
        }
    }
}

#[derive(Default, Resource)]
pub struct Timer {
    pub seconds_step: f32,
    pub seconds_24: f32
}

fn spawn_sun(mut commands: Commands) {
    commands.spawn(Sun::new(Transform::from_xyz(1000., 1000., 1000.))); 
}

fn cycle_daylight(mut commands: Commands, time: Res<Time>, mut timer: ResMut<Timer>, mut sun_query: Query<&mut DirectionalLight>) {
    let mut sun = sun_query.single_mut().unwrap();
    timer.seconds_step += time.delta_secs();
    if timer.seconds_step >= 0.1 {
        if timer.seconds_24 >= 24.0 {
            timer.seconds_24 = 0.;
        }
        timer.seconds_24 += timer.seconds_step;
        timer.seconds_step = 0.;
        let color = kelvin_to_rgb(bell_kelvin(timer.seconds_24));
        info!("Light update, value: {} color: {:?}", timer.seconds_24, color);
        sun.color = Color::srgb_u8(color.0, color.1, color.2);
    }
}

fn bell_kelvin(x: f32) -> f32 {
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

