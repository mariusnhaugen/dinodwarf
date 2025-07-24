use bevy::prelude::*;

pub struct TooltipsPlugin;

impl Plugin for TooltipsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_hitsplats);
        app.add_event::<HitsplatEvent>();
        app.add_systems(PostUpdate, event_listener);
    }
}

#[derive(Component)]
struct ScrollingText{
    timer: Timer,
}

#[derive(Event)]
pub struct HitsplatEvent {
    pub text: String,
    pub x: f32,
    pub y: f32,
}

fn event_listener(mut commands: Commands, mut events: EventReader<HitsplatEvent>) {
    for e in events.read() {
        commands.spawn((
            Text2d::new(format!("{}",e.text)),
            Transform::from_xyz(e.x ,e.y ,100.), //Z-index 100
            ScrollingText{timer: Timer::from_seconds( 1.0, TimerMode::Once,)},
            TextFont { font_size: 12.0, ..default()},
        ));
    } 
}

fn animate_hitsplats(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity,&mut ScrollingText,&mut Transform)>){
    for (entity, mut text, mut transform) in &mut query {
        text.timer.tick(time.delta());

        transform.translation.y = 120. * text.timer.elapsed_secs(); 

        if text.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}


