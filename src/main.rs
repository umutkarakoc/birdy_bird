mod animaton;
mod bird;
mod cam;
mod game;
mod obstacle;
mod ui;
mod world;

use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    // input::common_conditions::input_toggle_active,
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    text::TextSettings,
    window::{WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use bevy_easings::EasingsPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use cam::{RetroCameraBundle, RetroCameraPlugin};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States, SystemSet)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Birdy Bird".into(),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    resolution: WindowResolution::new(1200., 716.),
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    // backends: Some(Backends::VULKAN),
                    ..default()
                },
            }),
        // LogDiagnosticsPlugin::default(),
        // FrameTimeDiagnosticsPlugin,
        EasingsPlugin,
        RetroCameraPlugin,
        // WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Escape)),
    ))
    .insert_resource(TextSettings {
        allow_dynamic_font_size: true,
        ..default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .add_state::<GameState>()
    .add_systems(Startup, (init,))
    .add_plugins(())
    .add_plugins((
        game::GamePlugin,
        world::WorldPlugin,
        bird::BirdPlugin,
        obstacle::ObstaclePlugin,
        animaton::AnimationPlugin,
        ui::UIPlugin,
    ));

    app.run();
}

fn init(mut commands: Commands) {
    let mut cam = RetroCameraBundle::fixed_auto(1.0, 1200.0, 716.0);
    // let mut cam = Camera2dBundle::default();
    cam.transform.translation.z = 100.;
    commands.spawn(cam);
}
