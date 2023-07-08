use crate::info_timer::InfoTimer;
use crate::TERMINAL_VELOCITY;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct MovingEntityPlugin;

impl Plugin for MovingEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tick_moving_entities)
            .add_system(handle_input);
    }
}

// Component that marks an entity as being affected by kinematic movement every frame
#[derive(Component)]
pub struct MovingEntity {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl MovingEntity {
    pub const ZERO: Self = Self::new(Vec2::ZERO, Vec2::ZERO, Vec2::ZERO);

    pub const fn new(position: Vec2, velocity: Vec2, acceleration: Vec2) -> Self {
        Self {
            position,
            velocity,
            acceleration,
        }
    }

    pub fn with_acceleration(acceleration: Vec2) -> Self {
        Self::new(Vec2::ZERO, Vec2::ZERO, acceleration)
    }
}

pub fn tick_moving_entities(mut query: Query<(&mut MovingEntity, &mut Transform)>) {
    for (mut moving_entity, mut transform) in query.iter_mut() {
        let (mut position, mut velocity, acceleration) = (
            moving_entity.position,
            moving_entity.velocity,
            moving_entity.acceleration,
        );

        position += velocity;
        velocity += acceleration;

        if velocity.length() > TERMINAL_VELOCITY {
            velocity = velocity.normalize() * TERMINAL_VELOCITY;
        }

        moving_entity.position = position;
        moving_entity.velocity = velocity;

        transform.translation.x = moving_entity.position.x;
        transform.translation.y = moving_entity.position.y;
    }
}

// Component that marks an entity being able to be controlled by keyboard input
#[derive(Component)]
pub struct ControllableEntity;

pub fn handle_input(
    mut query: Query<&mut MovingEntity, With<ControllableEntity>>,
    input: Res<Input<KeyCode>>,
    info_timer: Res<InfoTimer>,
) {
    const INPUT_SENSITIVITY: f32 = 10.0;

    let up = Vec2::new(0.0, 1.0);
    let down = Vec2::new(0.0, -1.0);
    let left = Vec2::new(-1.0, 0.0);
    let right = Vec2::new(1.0, 0.0);

    for mut moving_entity in query.iter_mut() {
        let mut velocity = Vec2::ZERO;

        if input.just_pressed(KeyCode::W) {
            velocity += up;
        }
        if input.just_pressed(KeyCode::A) {
            velocity += left;
        }
        if input.just_pressed(KeyCode::S) {
            velocity += down;
        }
        if input.just_pressed(KeyCode::D) {
            velocity += right;
        }

        if velocity.length() != 0.0 {
            velocity = velocity.normalize();
        }

        velocity *= INPUT_SENSITIVITY;

        moving_entity.velocity += velocity;
    }
}
