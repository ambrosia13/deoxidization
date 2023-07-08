mod info_timer;
mod moving_entity;

use crate::moving_entity::{ControllableEntity, MovingEntity};
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_RADIUS: f32 = PLAYER_SIZE / 2.0;

pub const WALL_ELASTICITY: f32 = 0.9;
pub const TERMINAL_VELOCITY: f32 = 20.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.29, 0.31, 0.41)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Deoxidization".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(moving_entity::MovingEntityPlugin)
        .add_plugin(info_timer::InfoTimerPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(restrict_player_movement)
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    info!("Spawning player");

    // Unused for now but I'll use it later
    let _window = window_query
        .get_single()
        .expect("Couldn't unwrap window query");

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        MovingEntity::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 5.0),
            Vec2::new(0.0, -0.25),
        ),
        ControllableEntity,
    ));
}

pub fn restrict_player_movement(
    mut player_query: Query<&mut MovingEntity, With<ControllableEntity>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query
        .get_single()
        .expect("Couldn't unwrap window query");

    let width = window.width();
    let height = window.height();

    let bounds_x = (-width / 2.0 + PLAYER_RADIUS, width / 2.0 - PLAYER_RADIUS);
    let bounds_y = (-height / 2.0 + PLAYER_RADIUS, height / 2.0 - PLAYER_RADIUS);

    for mut entity in player_query.iter_mut() {
        let clamped_x = entity.position.x.clamp(bounds_x.0, bounds_x.1);
        let clamped_y = entity.position.y.clamp(bounds_y.0, bounds_y.1);

        if clamped_x != entity.position.x {
            entity.position.x = clamped_x;
            entity.velocity.x *= -WALL_ELASTICITY;
        }

        if clamped_y != entity.position.y {
            entity.position.y = clamped_y;
            entity.velocity.y *= -WALL_ELASTICITY;
        }
    }
}
