#[cfg(not(any(target_os = "horizon", feature = "horizon-input")))]
use bevy::{
    input::mouse::AccumulatedMouseMotion,
    window::{CursorGrabMode, CursorOptions},
};
use bevy::{prelude::*, window::PrimaryWindow};
use sr2_core::{Inventory, suction_target};
use sr2_platform::PlayerIntent;

pub struct PrototypePlugin;
impl Plugin for PrototypePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Controls>()
            .init_resource::<Pocket>()
            .add_systems(Startup, scene)
            .add_systems(Update, (input, player, slimes, vacpack, title).chain());
    }
}
#[derive(Resource, Default)]
struct Controls(PlayerIntent);
#[derive(Resource, Default)]
pub struct Pocket(pub Inventory);
#[derive(Component)]
struct Player {
    vertical: f32,
    yaw: f32,
    pitch: f32,
}
#[derive(Component)]
pub struct Slime {
    pub phase: f32,
}
#[derive(Resource)]
struct SlimeVisual {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

fn scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(60., 60.))),
        MeshMaterial3d(materials.add(Color::srgb(0.18, 0.4, 0.23))),
    ));
    // A solid obstacle, mirrored by the simple AABB player collision below.
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(3., 2., 3.))),
        MeshMaterial3d(materials.add(Color::srgb(0.35, 0.32, 0.25))),
        Transform::from_xyz(4., 1., -4.),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.4, 0.)),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 1.7, 5.),
        Player {
            vertical: 0.,
            yaw: 0.,
            pitch: 0.,
        },
    ));
    let visual = SlimeVisual {
        mesh: meshes.add(Sphere::new(0.55).mesh().uv(24, 16)),
        material: materials.add(Color::srgb(1., 0.28, 0.55)),
    };
    commands.spawn((
        Mesh3d(visual.mesh.clone()),
        MeshMaterial3d(visual.material.clone()),
        Transform::from_xyz(0., 0.55, 0.),
        Slime { phase: 0. },
    ));
    commands.insert_resource(visual);
}
#[cfg(not(any(target_os = "horizon", feature = "horizon-input")))]
fn input(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mouse: Res<AccumulatedMouseMotion>,
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut controls: ResMut<Controls>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }
    if keys.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
    let active = cursor.grab_mode != CursorGrabMode::None;
    controls.0 = PlayerIntent {
        movement: [
            (keys.pressed(KeyCode::KeyD) as u8 as f32) - (keys.pressed(KeyCode::KeyA) as u8 as f32),
            (keys.pressed(KeyCode::KeyW) as u8 as f32) - (keys.pressed(KeyCode::KeyS) as u8 as f32),
        ],
        look: if active {
            mouse.delta.to_array()
        } else {
            [0.; 2]
        },
        jump: keys.just_pressed(KeyCode::Space),
        suction: active && buttons.pressed(MouseButton::Left),
        launch: active && buttons.just_pressed(MouseButton::Right),
    };
}
fn player(
    time: Res<Time>,
    controls: Res<Controls>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let dt = time.delta_secs().min(0.05);
    for (mut t, mut p) in &mut query {
        p.yaw -= controls.0.look[0] * 0.002;
        p.pitch = (p.pitch - controls.0.look[1] * 0.002).clamp(-1.5, 1.5);
        t.rotation = Quat::from_euler(EulerRot::YXZ, p.yaw, p.pitch, 0.);
        let movement = Vec2::from_array(controls.0.movement).clamp_length_max(1.);
        let delta = Quat::from_rotation_y(p.yaw) * Vec3::new(movement.x, 0., -movement.y) * 5. * dt;
        let candidate = t.translation + delta;
        if !(candidate.x > 2.2
            && candidate.x < 5.8
            && candidate.z > -5.8
            && candidate.z < -2.2
            && t.translation.y < 3.7)
        {
            t.translation.x = candidate.x.clamp(-29., 29.);
            t.translation.z = candidate.z.clamp(-29., 29.);
        }
        if controls.0.jump && t.translation.y <= 1.701 {
            p.vertical = 6.;
        }
        p.vertical -= 18. * dt;
        t.translation.y += p.vertical * dt;
        if t.translation.y < 1.7 {
            t.translation.y = 1.7;
            p.vertical = 0.;
        }
    }
}
fn slimes(time: Res<Time>, mut query: Query<(&mut Transform, &mut Slime)>) {
    for (mut t, mut s) in &mut query {
        s.phase += time.delta_secs() * 2.5;
        t.translation.y = 0.55 + s.phase.sin().max(0.) * 0.9;
    }
}
fn vacpack(
    mut commands: Commands,
    time: Res<Time>,
    controls: Res<Controls>,
    mut pocket: ResMut<Pocket>,
    player: Single<&Transform, (With<Player>, Without<Slime>)>,
    mut slimes: Query<(Entity, &mut Transform), With<Slime>>,
    visual: Res<SlimeVisual>,
) {
    if controls.0.suction {
        for (entity, mut t) in &mut slimes {
            let offset = t.translation - player.translation;
            let distance = offset.length();
            if suction_target(distance, offset.normalize_or_zero().dot(*player.forward())) {
                t.translation -= offset.normalize_or_zero() * time.delta_secs().min(0.05) * 7.;
                if distance < 1.4 && pocket.0.capture() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
    if controls.0.launch && pocket.0.release() {
        let mut position = player.translation + player.forward() * 3.;
        position.y = 0.55;
        commands.spawn((
            Mesh3d(visual.mesh.clone()),
            MeshMaterial3d(visual.material.clone()),
            Transform::from_translation(position),
            Slime { phase: 0. },
        ));
    }
}
fn title(pocket: Res<Pocket>, mut window: Single<&mut Window, With<PrimaryWindow>>) {
    if pocket.is_changed() {
        window.title = format!(
            "SR2-NX prototype | Pink slimes: {}/20 | WASD Space LMB/RMB Esc",
            pocket.0.pink_slimes
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn harness() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .init_resource::<Controls>()
            .init_resource::<Pocket>()
            .insert_resource(SlimeVisual {
                mesh: Handle::default(),
                material: Handle::default(),
            });
        app.world_mut().spawn((
            Transform::from_xyz(0., 1.7, 0.),
            Player {
                vertical: 0.,
                yaw: 0.,
                pitch: 0.,
            },
        ));
        app
    }
    #[test]
    fn suction_captures_then_release_spawns_slime() {
        let mut app = harness();
        app.world_mut()
            .spawn((Transform::from_xyz(0., 1.7, -1.), Slime { phase: 0. }));
        app.add_systems(Update, vacpack);
        app.world_mut().resource_mut::<Controls>().0.suction = true;
        app.update();
        assert_eq!(app.world().resource::<Pocket>().0.pink_slimes, 1);
        assert_eq!(
            app.world_mut().query::<&Slime>().iter(app.world()).count(),
            0
        );
        app.world_mut().resource_mut::<Controls>().0 = PlayerIntent {
            launch: true,
            ..default()
        };
        app.update();
        assert_eq!(app.world().resource::<Pocket>().0.pink_slimes, 0);
        assert_eq!(
            app.world_mut().query::<&Slime>().iter(app.world()).count(),
            1
        );
    }
    #[test]
    fn player_jumps_lands_and_stops_at_obstacle() {
        let mut app = harness();
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
        app.add_systems(Update, player);
        app.world_mut().resource_mut::<Controls>().0.jump = true;
        app.update();
        let height = app
            .world_mut()
            .query_filtered::<&Transform, With<Player>>()
            .single(app.world())
            .unwrap()
            .translation
            .y;
        assert!(height > 1.7);
        app.world_mut().resource_mut::<Controls>().0.jump = false;
        for _ in 0..120 {
            app.update();
        }
        let player_entity = app
            .world_mut()
            .query_filtered::<Entity, With<Player>>()
            .single(app.world())
            .unwrap();
        assert!(
            (app.world()
                .get::<Transform>(player_entity)
                .unwrap()
                .translation
                .y
                - 1.7)
                .abs()
                < 0.001
        );
        app.world_mut()
            .get_mut::<Transform>(player_entity)
            .unwrap()
            .translation = Vec3::new(4., 1.7, -1.5);
        app.world_mut().resource_mut::<Controls>().0.movement = [0., 1.];
        for _ in 0..60 {
            app.update();
        }
        let z = app
            .world()
            .get::<Transform>(player_entity)
            .unwrap()
            .translation
            .z;
        assert!(
            z >= -2.2 && z < -1.5,
            "player must stop before the expanded obstacle: {z}"
        );
    }
    #[test]
    fn full_inventory_preserves_entity() {
        let mut app = harness();
        app.world_mut().resource_mut::<Pocket>().0.pink_slimes = Inventory::CAPACITY;
        let entity = app
            .world_mut()
            .spawn((Transform::from_xyz(0., 1.7, -1.), Slime { phase: 0. }))
            .id();
        app.world_mut().resource_mut::<Controls>().0.suction = true;
        app.add_systems(Update, vacpack);
        app.update();
        assert!(app.world().get_entity(entity).is_ok());
    }
}

#[cfg(any(target_os = "horizon", feature = "horizon-input"))]
fn input(
    time: Res<Time>,
    gamepads: Query<&Gamepad>,
    mut controls: ResMut<Controls>,
    mut exit: MessageWriter<AppExit>,
) {
    controls.0 = PlayerIntent::default();
    if let Some(pad) = gamepads.iter().next() {
        controls.0 = PlayerIntent {
            movement: pad.left_stick().to_array(),
            look: (pad.right_stick() * Vec2::new(480., -480.) * time.delta_secs().min(0.05))
                .to_array(),
            jump: pad.just_pressed(GamepadButton::South),
            suction: pad.pressed(GamepadButton::LeftTrigger2),
            launch: pad.just_pressed(GamepadButton::RightTrigger2),
        };
        if pad.just_pressed(GamepadButton::Start) {
            exit.write(AppExit::Success);
        }
    }
}
