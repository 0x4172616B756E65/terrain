use std::collections::HashMap;

use bevy::{app::{Plugin, Startup}, color::palettes::css::{DARK_GRAY, LIGHT_GREY}, ecs::bundle::Bundle, prelude::*, ui::{Node, Val}, window::PrimaryWindow};
use uuid::Uuid;
use tracing::{info, error};

use crate::player::{cursor::Cursor, inventory::{hud::load_hud, items::Item}, player::Player, player_state::ToggleInventory};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_event::<ToggleInventory>()
            .add_systems(Startup, load_hud)
            .add_systems(Update, (hover, toggle_inventory));
    }
}

pub struct Inventory<'a> {
    pub active: Option<Item>,

    pub physical: HashMap<(u8, u8), &'a Uuid>,
    pub logical: HashMap<Uuid, Item>
}

impl<'a> Inventory<'a> {
    fn new() -> Self {
        Inventory { 
            active: None, 

            physical: HashMap::new(), 
            logical: HashMap::new() 
        }
    }

    fn insert(&self, item: Item, cell: (u8, u8)) {
        if let Some(uuid) = &self.physical.get(&cell) {
           error!("Cell is occupied: {}", &self.logical.get(uuid).unwrap_or_default()); 
        }
    }

    fn swap(&self, cell: (u8, u8), source_cell: (u8, u8)) {

    }
}

fn toggle_inventory(
    player_query: Query<&Player>,
    mut commands: Commands, 
    mut event_reader: EventReader<ToggleInventory>,
    inventory_query: Query<Entity, With<InventoryUI>>,
) {
    let player = player_query.single().unwrap();
    let size = player.inventory_size;

    for _ in event_reader.read() {
        if !player.state.inventory_open {
            if inventory_query.is_empty() {
                commands.spawn((
                    Node {
                        justify_content: bevy::ui::JustifyContent::Center,
                        align_items: bevy::ui::AlignItems::Center,
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        ..Default::default()
                    },
                    InventoryUI,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            justify_content: bevy::ui::JustifyContent::Center,
                            align_items: bevy::ui::AlignItems::Center,
                            row_gap: Val::Px(20.0),
                            flex_wrap: FlexWrap::Wrap,
                            height: Val::Percent(100.0),
                            width: Val::Percent(70.0),
                            ..Default::default()
                        },
                        BackgroundColor(Color::srgba_u8(20, 20, 20, 180))
                    ))
                    .with_children(|grid| {
                        for y in 0..size.1 {
                            for x in 0..size.0 {
                                grid.spawn((
                                    Node {
                                        width: Val::Px(50.0),
                                        height: Val::Px(50.0),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.2, 0.2, 0.8, 0.6)),
                                    Interaction::default(),
                                    Hoverable,
                                ));
                            }
                        }
                    });
                });
            }
        } else {
            for entity in inventory_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

#[derive(Component)]
struct InventoryUI;
#[derive(Component)]
struct Hoverable;

fn hover(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut slot_query: Query<(&Interaction, &mut BackgroundColor), With<Hoverable>>,
) {
    let window = window_query.single().unwrap();
    if let Some(cursor_pos) = window.cursor_position() {
        for (interaction, mut background_color) in slot_query.iter_mut() {
            match interaction {
                Interaction::Hovered => *background_color = Color::srgba(0.5, 0.5, 1.0, 0.8).into(),
                Interaction::None => *background_color = Color::srgba(0.2, 0.2, 0.8, 0.6).into(),
                Interaction::Pressed => *background_color = Color::srgba(0.8, 0.2, 0.2, 0.8).into(),
            }
        }
    }
}
