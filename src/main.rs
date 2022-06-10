#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cargo_common_metadata,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::needless_pass_by_value,
    clippy::multiple_crate_versions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::similar_names,
    clippy::must_use_candidate,
    clippy::enum_glob_use
)]

use std::time::Duration;

use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
    window::PresentMode,
};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::*,
};
use bevy_tweening::{
    lens::TransformPositionLens, Animator, EaseFunction, Tween, TweeningPlugin, TweeningType,
};
use debug::Debug;

mod debug;

pub const CLEAR: Color = Color::BLACK;
pub const HEIGHT: f32 = 600.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Coin game".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        // External plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(TweeningPlugin)
        .add_plugins(DefaultPickingPlugins)
        // Internal plugins
        .add_plugin(Debug)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_board)
        .add_system(spawn_coins)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = PerspectiveCameraBundle::new_3d();

    camera.transform.translation = Vec3::new(0.0, 10.0, 10.0);
    camera.transform.look_at(Vec3::ZERO, Vec3::Y);

    commands
        .spawn_bundle(camera)
        .insert_bundle(PickingCameraBundle::default());
}

#[derive(Component)]
struct Pusher;

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    commands
        .spawn_bundle((Collider::cuboid(10.0, 1.0, 5.0), RigidBody::Fixed))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(shape::Box::new(20.0, 2.0, 10.0).into()),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        });
    commands
        .spawn_bundle((Collider::cuboid(1.0, 10.0, 25.0), RigidBody::Fixed))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(shape::Box::new(2.0, 10.0, 50.0).into()),
            material: materials.add(Color::GOLD.into()),
            transform: Transform::from_xyz(-5.0, 0.0, 0.0),
            ..default()
        });
    commands
        .spawn_bundle((Collider::cuboid(1.0, 10.0, 25.0), RigidBody::Fixed))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(shape::Box::new(2.0, 10.0, 50.0).into()),
            material: materials.add(Color::GOLD.into()),
            transform: Transform::from_xyz(5.0, 0.0, 0.0),
            ..default()
        });
    let transform = Transform::from_xyz(0.0, 0.0, -13.);
    commands
        .spawn_bundle((
            Collider::cuboid(10.0, 1.0, 10.0),
            RigidBody::KinematicPositionBased,
            Pusher,
            Name::new("Pusher"),
            Animator::new(Tween::new(
                EaseFunction::SineInOut,
                TweeningType::PingPong,
                Duration::from_secs(3),
                TransformPositionLens {
                    start: transform.translation,
                    end: Vec3::new(0.0, 0.0, -15.),
                },
            )),
        ))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(shape::Box::new(20.0, 2.0, 20.0).into()),
            material: materials.add(Color::GOLD.into()),
            transform,
            ..default()
        })
        .insert_bundle(PickableBundle::default());

    for x in -4..4 {
        spawn_coin(
            &mut commands,
            Vec3::new(1.01 * x as f32 + 0.5, 0.05, 0.0),
            &ass,
        );
    }
    for x in -3..4 {
        spawn_coin(&mut commands, Vec3::new(1.01 * x as f32, 0.05, -1.0), &ass);
    }
    for x in -4..4 {
        spawn_coin(
            &mut commands,
            Vec3::new(1.01 * x as f32 + 0.5, 0.05, -2.0),
            &ass,
        );
    }
}

fn spawn_coins(
    mut commands: Commands,
    mut events: EventReader<MouseButtonInput>,
    ass: Res<AssetServer>,
) {
    for event in events.iter() {
        if let MouseButtonInput {
            button: MouseButton::Left,
            state: ElementState::Pressed,
        } = event
        {
            spawn_coin(&mut commands, Vec3::new(0.0, 2.0, -4.0), &ass);
        }
    }
}

fn spawn_coin(commands: &mut Commands, position: Vec3, ass: &Res<AssetServer>) {
    commands
        .spawn_bundle((Collider::cylinder(0.05, 0.5), RigidBody::Dynamic))
        .insert_bundle(PbrBundle {
            mesh: ass.load("coin.glb#Mesh0/Primitive0"),
            material: ass.load("coin.glb#Material0"),
            transform: Transform::from_translation(position),
            ..default()
        });
}
