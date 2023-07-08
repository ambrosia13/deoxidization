use bevy::prelude::*;

pub struct InfoTimerPlugin;

impl Plugin for InfoTimerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InfoTimer>().add_system(tick_timer);
    }
}

#[derive(Resource)]
pub struct InfoTimer {
    pub timer: Timer,
}

impl Default for InfoTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

fn tick_timer(mut info_timer: ResMut<InfoTimer>, time: Res<Time>) {
    info_timer.timer.tick(time.delta());
}
