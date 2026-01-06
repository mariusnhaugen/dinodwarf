use bevy::prelude::*;

use crate::{resources::{ResourceEvent, ResourceType}, tooltips::HitsplatEvent};


#[derive(Component)]
struct Dwarf;

#[derive(Component)]
struct MineTimer{
    timer: Timer,
}

#[derive(Event)]
pub struct DwarfEvent {
    price: i32,
}


pub struct DwarfPlugin;

impl Plugin for DwarfPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_dwarf);
        app.add_systems(Update, mine_stone);
    }
}




fn spawn_dwarf(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
            Dwarf,
            Sprite::from_image(asset_server.load("dwarf.png")),
            Transform::from_xyz(80., 0., 100.),
            MineTimer{timer: Timer::from_seconds( 3.0, TimerMode::Repeating,)},
        ));
}

fn mine_stone(mut res_event: EventWriter<ResourceEvent>, time: Res<Time>,mut query: Query<(&mut MineTimer, &Transform)>,mut hs_event: EventWriter<HitsplatEvent> ){
    let stone_added = 6;
   
    for (mut mine_timer, transform) in &mut query {
        mine_timer.timer.tick(time.delta());
        
        let hitsplat_text = format!("+{:?}", stone_added);
        let dwarf_x = transform.translation.x;
        let dwarf_y = transform.translation.y;

        if mine_timer.timer.finished() {
            res_event.write(ResourceEvent {resource: ResourceType::Stone , amount: stone_added});
            hs_event.write(HitsplatEvent { text: hitsplat_text, x: dwarf_x, y: dwarf_y });
        }
    } 
}

