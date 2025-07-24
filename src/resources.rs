use bevy::prelude::*;


#[derive(Component)]
struct ResourceDisplay;

#[derive(Resource,Default, Debug)]
pub struct Resources {
    pub stone: u32,
    pub wood: u32,
    pub gold: u32
}

pub enum ResourceType {
    Stone,
    Wood,
    Gold 
}

#[derive(Event)]
pub struct ResourceEvent {
    pub r#type: ResourceType,
    pub amount: i32,
}

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(Resources::default());
        app.add_systems(Update, display_resources);
        app.add_event::<ResourceEvent>();
    }
}
fn display_resources(mut commands: Commands, res: Res<Resources>, query: Query<(Entity, &ResourceDisplay)>){
    //clear text
    for (entity, _) in query.iter(){
            commands.entity(entity).despawn();
    }
    
    //spawn text 
    commands.spawn((
        Text::new(format!("Stone: {}", res.stone)),
        TextFont{font_size: 14.0, ..default()},
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(25.0),
            left: Val::Px(25.0),
            ..default()
        },
        ResourceDisplay,
    ));
}


