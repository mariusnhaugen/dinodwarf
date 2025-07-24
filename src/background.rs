use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_background);
        //app.add_systems(Update, animate);
    }
}

const TILE_SIZE: u32= 32;
const FLOOR_LEVEL: f32 = -(TILE_SIZE as f32);  
const DIRT_LEVEL: f32 = 1.5*FLOOR_LEVEL-(DIRT_HEIGHT/2.);
const DIRT_HEIGHT: f32 =30.*(TILE_SIZE as f32);  
const BACKGROUND_WIDTH: f32 = 80.*(TILE_SIZE as f32);  


#[derive(Component)]
struct BackgroundMarker;




fn load_background(mut commands: Commands, asset_server: Res<AssetServer>, mut tex_layouts: ResMut<Assets<TextureAtlasLayout>>) {
   
    //Grass
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 1, None, None);
    let texture_atlas_layout = tex_layouts.add(layout);
    commands.spawn((
        BackgroundMarker,
        Sprite {
            image: asset_server.load("terrain.png"),
            texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                }), 
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0
            },
            custom_size: Some(Vec2 {x: BACKGROUND_WIDTH, y:TILE_SIZE as f32}),
            ..default()

        },
        Transform::from_xyz(0., FLOOR_LEVEL, -100.)
    ));
   

    //Dirt 
    let layout_dirt = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 1, None, None);
    let tal_dirt = tex_layouts.add(layout_dirt);
    commands.spawn((
        BackgroundMarker,
        Sprite {
            image: asset_server.load("terrain.png"),
            texture_atlas: Some(TextureAtlas {
                    layout: tal_dirt,
                    index: 1,
                }), 
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0
            },
            custom_size: Some(Vec2 {x: BACKGROUND_WIDTH, y: DIRT_HEIGHT}),
            ..default()

        },
        Transform::from_xyz(0., DIRT_LEVEL, 0.)
    ));

}

