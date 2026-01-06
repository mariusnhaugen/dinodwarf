use bevy::prelude::*;

use crate::resources::{ResourceEvent, ResourceType};
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

fn onclick_give_stone(_click: Trigger<Pointer<Click>>, mut hs_event: EventWriter<HitsplatEvent>, mut res_event: EventWriter<ResourceEvent>) {
    let stone_added = calculate_player_stone_generated();
  
    let swing = 50.;
    let rand_x: f32 = rand::random_range(-swing..=swing);
    let hitsplat_text = format!("+{:?}", stone_added);
    hs_event.write(HitsplatEvent { text: hitsplat_text ,x: rand_x, y: 0. });
    res_event.write(ResourceEvent {resource: ResourceType::Stone , amount: stone_added});
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


fn calculate_player_stone_generated() -> i32 {
    let multiplier = 4;
    1*multiplier
}

