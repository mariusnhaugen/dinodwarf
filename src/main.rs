mod resources;
mod stones;
mod background;
mod tooltips;
mod dwarf;
mod userinterface;

use bevy::prelude::*;
use bevy::window::WindowResolution;

//Third Party Plugins
use bevy_pancam::*;
//Custom Plugins
use stones::StonesPlugin;
use resources::ResourcesPlugin;
use background::BackgroundPlugin;
use tooltips::TooltipsPlugin;
use dwarf::DwarfPlugin;
use userinterface::UIPlugin;


//Resolution: 640x360 (or 320x180) scales evenly for all common* (?) screen resolution 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dinodwarf".into(),
                resolution:WindowResolution::new(1280.,720.).with_scale_factor_override(2.0),  
                //mode: WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current),
                visible: true, //can be false and toggled after frames have started spawning
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup_camera)
        .insert_resource(ClearColor(Color::hsl(210., 1., 0.8)))
        //Third Party Plugins
        .add_plugins(PanCamPlugin)
        //Custom Plugins 
        .add_plugins(ResourcesPlugin)
        .add_plugins(StonesPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(TooltipsPlugin)
        .add_plugins(DwarfPlugin)
        .add_plugins(UIPlugin)
        .run();

}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), PanCam {
        max_scale: 40.,
        min_scale: 0.5,
        min_y: -1000.,
        max_y: 500.,
        min_x: -1000.,
        max_x: 1000.,
        ..default()
    }));
}



