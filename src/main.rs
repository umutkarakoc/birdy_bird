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
    render::{camera::ScalingMode, settings::WgpuSettings, RenderPlugin},
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
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 100.),
        projection: OrthographicProjection {
            scale: 1.,

            scaling_mode: ScalingMode::FixedVertical(716.),

            ..Default::default()
        },
        ..Default::default()
    });
}
