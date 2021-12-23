extern crate rand;

mod boid;
mod constants;
mod point;
mod vector;
mod world;

use piston_window::*;
use world::World;

const NUM_BOIDS: u32 = 250;
const SIZE: u32 = 600;

const BOID_BOD: &'static [[f64; 2]] = &[[5.0, 5.0], [10.0, 0.0], [5.0, 15.0], [0.0, 0.0]];

fn main() {
    let mut the_flock = World::new(NUM_BOIDS, SIZE as f32);
    println!(
        "=== Flock of Seaboids with Piston ===\n {} [version {}]",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
    );
    let mut window: PistonWindow = WindowSettings::new("flock-of-boids", (SIZE, SIZE))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let i = 1.1;
    while let Some(e) = window.next() {
        window.draw_2d(&e, |context, gfx, _| {
            clear([1.0, 1.0, 1.0, 1.0], gfx);

            the_flock.step(i);
            let boids = the_flock.boids();
            for i in 0..boids.len() {
                let boid = boids[i];
                let point = boid.point();
                let transform = context
                    .transform
                    .trans(point.x() as f64, point.y() as f64)
                    .rot_rad(-boid.angle() as f64);

                polygon(boid.color, BOID_BOD, transform, gfx);
            }
        });
    }
}
