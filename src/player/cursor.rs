use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .add_systems(Startup, init_cursor);
    }
}

#[derive(Resource, Default)]
pub struct Cursor {
    locked: bool
}

impl Cursor {
    pub fn invert_lock(&mut self, window: &mut Mut<'_, Window>) {
        self.locked = !self.locked;
        window.cursor_options.visible = !self.locked;
        if self.locked {
            let width = window.width();
            let height = window.height();
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.set_cursor_position(Some(Vec2 { x: width / 2., y: height / 2. }));
        } else {
            window.cursor_options.grab_mode = CursorGrabMode::None;
        }
    }
}

fn init_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>, mut cursor: ResMut<Cursor>) {
    let mut window = window_query.single_mut().unwrap();
    cursor.invert_lock(&mut window);
}
