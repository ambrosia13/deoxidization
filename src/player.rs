use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use crate::globals::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
            transform: Transform {
                translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(KinematicCharacterController::default());
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();

    let mut movement = 0.0;

    if input.pressed(KeyCode::Right) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X;
    }

    if input.pressed(KeyCode::Left) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
    }

    player.translation = match player.translation {
        Some(translation) => Some(Vec2::new(movement, translation.y)),
        None => Some(Vec2::new(movement, 0.0)),
    }
}

#[derive(Component)]
pub struct Jump(f32);

pub fn jump_begin(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();

    if input.pressed(KeyCode::Up) && output.grounded {
        commands.entity(player).insert(Jump(0.0));
    }
}

pub fn jump_rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut player, mut jump) = query.single_mut();

    let mut movement = time.delta().as_secs_f32() * PLAYER_VELOCITY_Y;
    movement *= ((MAX_JUMP_HEIGHT - jump.0) / MAX_JUMP_HEIGHT)
        .powf(1.1)
        .max(0.25);

    if movement + jump.0 >= MAX_JUMP_HEIGHT {
        movement = MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    jump.0 += movement;

    player.translation = match player.translation {
        Some(translation) => Some(Vec2::new(translation.x, movement)),
        None => Some(Vec2::new(0.0, movement)),
    };
}

pub fn jump_fall(
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController, Without<Jump>>,
) {
    if query.is_empty() {
        return;
    }

    let mut player = query.single_mut();

    let mut movement = -time.delta().as_secs_f32() * PLAYER_VELOCITY_Y;

    player.translation = match player.translation {
        Some(translation) => Some(Vec2::new(translation.x, movement)),
        None => Some(Vec2::new(0.0, movement)),
    };
}
