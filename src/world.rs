use crate::{boid::Boid, point::Point, vector::Vector};
use rand::Rng;

#[derive(Clone)]
pub struct World {
    width: f32,
    height: f32,
    boids: Vec<Boid>,
}

struct Grid {
    x: f32,
    y: f32,
}

const MAX_VELOCITY: f32 = 2.0;
const MIN_VELOCITY: f32 = 0.5;

const SIGHT: f32 = 25.0;
const GRID_GAP: f32 = 8.0;
const FIELD_OF_VIEW: f32 = std::f32::consts::PI * 3.0 / 4.0;

impl World {
    pub fn new(total_boids: u32, size: f32) -> World {
        let mut boids = Vec::new();
        let step = size / total_boids as f32;
        let mut rng = rand::thread_rng();
        for i in 0..total_boids {
            let v: f32 = i as f32 * step + 1f32;
            let point = Point::new(
                rng.gen_range(MIN_VELOCITY..v),
                rng.gen_range(MIN_VELOCITY..v),
            );
            let vector = Vector {
                dx: rng.gen_range(MIN_VELOCITY..MAX_VELOCITY),
                dy: rng.gen_range(MIN_VELOCITY..MAX_VELOCITY),
            };
            boids.push(Boid::new(point, vector, i));
        }

        World {
            width: size,
            height: size,
            boids: boids,
        }
    }

    pub fn step(&mut self, seconds: f32) {
        for i in 0..self.boids.len() {
            let mut boid = self.boids[i];
            let neighbors = self.clone().visible_neighbors(&boid);
            boid.step(seconds, neighbors);
            boid.bound(self.width, self.height);
            self.boids[i] = boid;
        }
    }

    pub fn visible_neighbors(&self, boid: &Boid) -> Vec<Boid> {
        let grid = Grid {
            x: (boid.point.x() / SIGHT).floor(),
            y: (boid.point.y() / SIGHT).floor(),
        };
        self.boids
            .iter()
            .cloned()
            .filter(|b| {
                if b.id == boid.id {
                    return false;
                }

                let other_grid = Grid {
                    x: (b.point.x() / SIGHT).floor(),
                    y: (b.point.y() / SIGHT).floor(),
                };

                if (grid.x - other_grid.x).abs() + (grid.y - other_grid.y).abs() > GRID_GAP {
                    return false;
                }

                let vector: Vector = boid.point.vector_to(&b.point);
                if vector.length() > SIGHT {
                    return false;
                }

                if vector.radial_distance(boid.vector) > FIELD_OF_VIEW {
                    return false;
                }

                true
            })
            .collect::<Vec<Boid>>()
    }

    pub fn boids(&self) -> Vec<Boid> {
        self.boids.clone()
    }
}
