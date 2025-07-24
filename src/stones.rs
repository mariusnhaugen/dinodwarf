use bevy::prelude::*;

use crate::resources::Resources;
use crate::tooltips::HitsplatEvent;

#[derive(Component)]
struct Stones;

pub struct StonesPlugin;

impl Plugin for StonesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_stones);
        app.add_systems(Update, cleanup_texts);
    }
}


fn spawn_stones(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
            Stones,
            Sprite::from_image(asset_server.load("stones.png")),
            Transform::from_xyz(0., -1.5, 100.),
            Pickable::default(),
        ))
        .observe(onclick_give_stone);
}

#[derive(Component)]
struct ScrollingText{
    timer: Timer,
}

fn onclick_give_stone(_click: Trigger<Pointer<Click>>, mut resources: ResMut<Resources>, mut my_events: EventWriter<HitsplatEvent>) {
    let stone_added = calculate_player_stone_generated();
  
    let swing = 50.;
    let rand_x: f32 = rand::random_range(-swing..=swing);
    let hitsplat_text = format!("+{:?}", stone_added);
    my_events.send(HitsplatEvent { text: hitsplat_text ,x: rand_x, y: 0. });
//    my_events.send(ResourceEvent {resource: Resource::Stone , amount:});
    
    resources.stone += stone_added;
}


fn cleanup_texts(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity,&mut ScrollingText,&mut Transform)>){
    for (entity, mut text, mut transform) in &mut query {
        text.timer.tick(time.delta());

        transform.translation.y = 120. * text.timer.elapsed_secs(); 

        if text.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}


fn calculate_player_stone_generated() -> u32 {
    let multiplier = 4;
    1*multiplier
}

