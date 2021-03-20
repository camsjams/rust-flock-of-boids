extern crate rand;

mod boid;
mod constants;
mod point;
mod vector;
mod world;

use tetra::graphics::{self, Color};
use tetra::graphics::{
    mesh::{GeometryBuilder, Mesh, ShapeStyle},
    DrawParams,
};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};
use world::World;

const NUM_BOIDS: u32 = 250;
const SIZE: i32 = 600;

struct GameState {
    flock: World,
    bird: Mesh,
}

const BOID_BOD: &'static [Vec2<f32>] = &[
    Vec2::new(5.0, 5.0),
    Vec2::new(10.0, 0.0),
    Vec2::new(5.0, 15.0),
    Vec2::new(0.0, 0.0),
];
impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            flock: World::new(NUM_BOIDS, SIZE as f32),
            bird: GeometryBuilder::new()
                .set_color(Color::BLACK)
                .polygon(ShapeStyle::Stroke(2.0), BOID_BOD)?
                .build_mesh(ctx)?,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::WHITE);
        self.flock.step(1.1);
        let boids = self.flock.get_boids();
        for i in 0..boids.len() {
            let boid = boids[i];
            let point = boid.get_point();

            //     .rot_rad(-boid.get_angle() as f64);

            self.bird.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(point.get_x(), point.get_y()))
                    .rotation(-boid.get_angle()),
            );
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    println!(
        "=== Flock of Seaboids with Tetra ===\n {} [version {}]",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
    );

    ContextBuilder::new("flock-of-boids", SIZE, SIZE)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
