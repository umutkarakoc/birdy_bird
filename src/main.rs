mod animaton;
mod bird;
mod cam;
mod game;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::common_conditions::input_toggle_active,
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
    text::TextSettings,
    window::{WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use bevy_easings::EasingsPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
                    title: "Wizard Castle".into(),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    resolution: WindowResolution::new(1200., 715.),
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    // backends: Some(Backends::GL),
                    ..default()
                },
            }),
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin,
    ))
    .insert_resource(TextSettings {
        allow_dynamic_font_size: true,
        ..default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .add_state::<GameState>()
    .add_systems(Startup, (init,))
    .add_plugins((
        EasingsPlugin,
        RetroCameraPlugin,
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Escape)),
    ))
    .add_plugins((
        game::GamePlugin,
        bird::BirdPlugin,
        animaton::AnimationPlugin,
    ));

    app.run();
}

fn init(mut commands: Commands) {
    let mut cam = RetroCameraBundle::fixed_auto(1.0, 1200.0, 715.0);
    // let mut cam = Camera2dBundle::default();
    cam.transform.translation.z = 100.;
    commands.spawn(cam);
}
