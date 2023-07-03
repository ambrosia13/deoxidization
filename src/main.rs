mod globals;
mod platform;
mod player;

use globals::*;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResolution;

use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Deoxidization".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(platform::PlatformsPlugin)
        .add_startup_system(create_floor)
        .add_startup_system(player::spawn_player)
        .add_system(player::movement)
        .add_system(player::jump_begin)
        .add_system(player::jump_rise)
        .add_system(player::jump_fall)
        .run();
}

fn create_floor(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_FLOOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + (FLOOR_THICKNESS / 2.0), 0.0),
                scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));

    commands.spawn(Camera2dBundle::default());
}
