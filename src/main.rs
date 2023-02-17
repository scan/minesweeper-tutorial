use bevy::prelude::*;
use board_plugin::BoardPlugin;
use board_plugin::resources::BoardOptions;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Mine Sweeper".to_string(),
            width: 700.0,
            height: 800.0,
            ..Default::default()
        },
        ..Default::default()
    }));

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin);

    app.add_plugin(BoardPlugin);
    app.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        ..Default::default()
    });

    app.add_startup_system(camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
