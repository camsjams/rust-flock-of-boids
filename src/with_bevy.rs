extern crate rand;

mod boid;
mod constants;
mod point;
mod vector;
mod world;

use bevy::prelude::*;
use world::World;

const NUM_BOIDS: u32 = 250;
const SIZE: i32 = 400;
const BOID_SIZE: f32 = 60.0;

struct GameState {
    flock: World,
}

struct Bird {
    id: u32,
}

fn main() {
    println!(
        "=== Flock of Seaboids with Bevy ===\n {} [version {}]",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
    );

    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(GameState {
            flock: World::new(NUM_BOIDS, SIZE as f32),
        })
        .add_startup_system(setup.system())
        .add_system(step_system.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: ResMut<GameState>,
) {
    let texture_handle = asset_server.load("sprites/bird.png");

    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    let boids = game_state.flock.get_boids();
    for boid in boids {
        let point = boid.get_point();

        let mut transform =
            Transform::from_translation(Vec3::new(point.get_x(), point.get_y(), 0.0));
        transform.rotate(Quat::from_rotation_z(-boid.get_angle()));
        commands
            .spawn((Bird { id: boid.id },))
            .with_bundle(SpriteBundle {
                sprite: Sprite {
                    size: Vec2::new(0.5, 0.5) * BOID_SIZE,
                    resize_mode: SpriteResizeMode::Manual,
                },
                material: materials.add(ColorMaterial {
                    texture: Some(texture_handle.clone()),
                    color: Color::BLACK,
                }),
                transform,
                ..SpriteBundle::default()
            });
    }
}

fn step_system(mut q: Query<(&Bird, &mut Transform)>, mut game_state: ResMut<GameState>) {
    game_state.flock.step(1.1);
    let boids = game_state.flock.get_boids();

    for (bird, mut transform) in q.iter_mut() {
        let boid = boids[bird.id as usize]; // <- could totally panic!
        let point = boid.get_point();
        transform.translation = Vec3::new(point.get_x(), point.get_y(), 0.0);
        transform.rotate(Quat::from_rotation_z(-boid.get_angle()));
    }
}
