use crate::globals;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const COLOR_PLATFORM: Color = Color::rgb(0.13, 0.13, 0.23);

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PlatformBundle::new(-100.0, Vec3::new(75.0, 200.0, 1.0)));
    commands.spawn(PlatformBundle::new(100.0, Vec3::new(50.0, 350.0, 1.0)));
    commands.spawn(PlatformBundle::new(350.0, Vec3::new(150.0, 250.0, 1.0)));
}

#[derive(Bundle)]
pub(crate) struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    pub fn new(x: f32, scale: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR_PLATFORM,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x, globals::WINDOW_BOTTOM_Y + scale.y / 2.0, 0.0),
                    scale,
                    ..default()
                },
                ..default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}
