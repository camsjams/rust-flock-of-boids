use crate::{constants::PI_X_2, vector::Vector};

#[derive(Clone, Copy)]
pub struct Boid {
    pub id: u32,
    pub point: Vector,
    pub velocity: Vector,
    pub color: [f32; 4],
}

const BOID_COLOR: [f32; 4] = [0.05, 0.05, 0.05, 0.7];

impl Boid {
    pub fn new(point: Vector, velocity: Vector, id: u32) -> Boid {
        Boid {
            id,
            point,
            velocity,
            color: BOID_COLOR,
        }
    }

    pub fn bound(&mut self, width: f32, height: f32) {
        self.point.bound(width, height);
    }

    pub fn get_angle(&self) -> f32 {
        self.velocity.get_angle()
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.velocity.set_angle(angle);
    }

    pub fn step_forward(&mut self, percent: f32) {
        self.point += self.velocity * percent;
    }

    pub fn get_point(self) -> Vector {
        self.point
    }

    pub fn step(&mut self, seconds: f32, neighbors: &[Boid]) {
        if !neighbors.is_empty() {
            let mut vectors: Vec<Vector> = Vec::new();

            let mut separation = Vector::mean(neighbors.iter().map(|b| b.point - self.point));
            separation.set_length(separation.get_length() + 15f32);
            vectors.push(separation);

            let average_location = Vector::mean(neighbors.iter().map(|b| b.point));
            vectors.push(average_location - self.point);

            let average_heading = Vector::mean(neighbors.iter().map(|b| {
                let mut v = Vector { x: 1f32, y: 0f32 };
                v.set_angle(b.velocity.get_angle());
                v.set_length(25f32);

                v
            }));
            vectors.push(average_heading);

            let final_vector = Vector::mean(vectors);
            self.turn_to(final_vector.get_angle(), 0.02f32);
        }

        self.step_forward(seconds);
    }

    pub fn turn_to(&mut self, mut heading: f32, percent: f32) {
        let angle = self.get_angle();
        if heading < angle {
            heading += PI_X_2;
        }
        let mut diff = heading - angle;

        if diff >= std::f32::consts::PI {
            diff -= PI_X_2;
        }

        self.set_angle(angle + diff * percent);
    }
}
