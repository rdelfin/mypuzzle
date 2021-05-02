use amethyst::{
    assets::PrefabLoaderSystemDesc,
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod components;
mod input;
mod prefabs;
mod state;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let bindings = app_root.join("config").join("bindings.ron");
    let display_config = app_root.join("config").join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<prefabs::PlainPrefabData>::default(),
            "plain_prefab_loader",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<prefabs::RotatingPrefab>::default(),
            "rotating_prefab_loader",
            &[],
        )
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<input::GameBindingTypes>::new().with_bindings_from_file(bindings)?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderShaded3D::default()),
        )?
        .with(systems::PhysicsSystem, "physics_system", &[])
        .with(
            systems::RotateInputSystem::default(),
            "rotate_input_system",
            &["input_system"],
        )
        .with(
            systems::RotateSystem,
            "rotate_system",
            &["rotate_input_system", "physics_system"],
        )
        .with(
            systems::CameraTrackSystem,
            "camera_track_system",
            &["rotate_system"],
        );

    let mut game = Application::new(resources, state::GameState, game_data)?;
    game.run();

    Ok(())
}
