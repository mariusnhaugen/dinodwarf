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
    pub resource: ResourceType,
    pub amount: i32,
}

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(Resources::default());
        app.add_systems(Update, display_resources);
        app.add_event::<ResourceEvent>();
        app.add_systems(PostUpdate, resource_event_listener);
    }
}


impl Resources {
    pub fn apply_event(&mut self, e: &ResourceEvent) {
            match e.resource {
                ResourceType::Stone => {
                    self.stone = Self::apply_amount(self.stone, e.amount);
                }
                ResourceType::Wood => {
                    self.wood = Self::apply_amount(self.wood, e.amount);
                }
                ResourceType::Gold=> {
                    self.wood = Self::apply_amount(self.gold, e.amount);
                }
            }
    }

    fn apply_amount(current: u32, delta: i32) -> u32 {
        if delta >= 0 {
            current.saturating_add(delta as u32)
        } else {
            current.saturating_sub(-(delta) as u32)
        }
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

fn resource_event_listener(mut res: ResMut<Resources>, mut events: EventReader<ResourceEvent>) {
    for e in events.read() {
        res.apply_event(e);    
    } 
}

