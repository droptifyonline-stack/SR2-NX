use bevy::prelude::*;
use sr2_gameplay::PrototypePlugin;
fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SR2-NX prototype".into(),
                    resolution: (1280, 720).into(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: sr2_assets::asset_root().to_string_lossy().into_owned(),
                ..default()
            }),
    )
    .init_resource::<ImportedModels>()
    .add_plugins(PrototypePlugin)
    .add_systems(Startup, converted_scene);
    if std::env::args().any(|arg| arg == "--smoke") {
        app.insert_resource(SmokeFrames(0))
            .add_systems(Update, smoke);
    }
    app.run();
}
#[derive(Resource)]
struct SmokeFrames(u32);
fn smoke(
    mut commands: Commands,
    mut frames: ResMut<SmokeFrames>,
    mut exit: MessageWriter<AppExit>,
    slimes: Query<&sr2_gameplay::Slime>,
    imported: Query<Option<&Children>, With<SceneRoot>>,
    clips: Res<Assets<AnimationClip>>,
) {
    frames.0 += 1;
    if frames.0 == 90 {
        if let Some(path) = std::env::var_os("SR2_SMOKE_SCREENSHOT") {
            commands
                .spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                .observe(bevy::render::view::screenshot::save_to_disk(
                    std::path::PathBuf::from(path),
                ));
        }
    }
    if frames.0 == 120 {
        assert_eq!(slimes.iter().count(), 1);
        if std::env::var_os("SR2_SCENE_FILE").is_some() {
            assert!(
                !imported.is_empty(),
                "converted scene must contain an instance"
            );
            assert!(
                imported
                    .iter()
                    .all(|children| children.is_some_and(|c| !c.is_empty())),
                "converted scenes must finish spawning before smoke completes"
            );
        }
        if std::env::var_os("SR2_SMOKE_EXPECT_ANIMATION").is_some() {
            assert!(!clips.is_empty(), "expected an imported animation clip");
        }
        info!(
            "SMOKE: 120 frames completed; placeholder slime present; {} animation clips loaded",
            clips.len()
        );
        exit.write(AppExit::Success);
    }
}

#[derive(Resource, Default)]
struct ImportedModels {
    handles: Vec<Handle<bevy::gltf::Gltf>>,
}
fn converted_scene(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut models: ResMut<ImportedModels>,
) {
    let Some(file) = std::env::var_os("SR2_SCENE_FILE") else {
        return;
    };
    let result = (|| -> Result<sr2_assets::SceneDescription, Box<dyn std::error::Error>> {
        let scene: sr2_assets::SceneDescription =
            serde_json::from_reader(std::fs::File::open(file)?)?;
        scene.validate(&sr2_assets::asset_root())?;
        Ok(scene)
    })();
    match result {
        Ok(scene) => {
            for item in scene.instances {
                // Keep the full glTF asset alive so its animation clips remain available.
                models
                    .handles
                    .push(server.load(item.model.to_string_lossy().into_owned()));
                commands.spawn((
                    SceneRoot(
                        server.load(
                            GltfAssetLabel::Scene(0)
                                .from_asset(item.model.to_string_lossy().into_owned()),
                        ),
                    ),
                    Transform::from_translation(Vec3::from_array(item.translation))
                        .with_rotation(Quat::from_rotation_y(item.rotation_y))
                        .with_scale(Vec3::splat(item.scale)),
                ));
            }
        }
        Err(error) => panic!("Converted scene validation failed: {error}"),
    }
}
