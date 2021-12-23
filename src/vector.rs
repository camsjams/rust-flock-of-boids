use crate::constants::PI_X_2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        Self { x, y }
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Self { x, y }
    }
}

impl std::ops::Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Self { x, y }
    }
}

impl std::ops::MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Vector {
    const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn mean(vectors: impl IntoIterator<Item = Vector>) -> Vector {
        let mut sum = Self::ZERO;
        let mut count = 0_usize;

        for vector in vectors {
            sum += vector;
            count += 1;
        }

        sum / count as f32
    }

    pub fn get_angle(&self) -> f32 {
        let mut angle = self.y.atan2(self.x);

        if angle < 0.0 {
            angle += PI_X_2;
        }

        angle
    }

    pub fn set_angle(&mut self, angle: f32) {
        let length = self.get_length();
        let rise = angle.sin() * length;
        let run = angle.cos() * length;

        self.y = rise;
        self.x = run;
    }

    pub fn get_length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn set_length(&mut self, value: f32) {
        let stretch = value / self.get_length();
        *self *= stretch;
    }

    pub fn radial_distance(&self, other: Vector) -> f32 {
        let diff = (other.get_angle() - self.get_angle()).abs();
        let diff2 = PI_X_2 - diff;

        diff.min(diff2)
    }

    pub fn bound(&mut self, width: f32, height: f32) {
        let mut x = self.x % width;
        let mut y = self.y % height;

        if x < 0.0 {
            x += width;
        }

        if y < 0.0 {
            y += height;
        }

        self.x = x;
        self.y = y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_five_vectors_returns_mean() {
        // arrange
        let input: Vec<Vector> = vec![
            Vector { x: 1.0, y: 10.0 },
            Vector { x: 2.0, y: 20.0 },
            Vector { x: 3.0, y: 30.0 },
            Vector { x: 4.0, y: 40.0 },
            Vector { x: 5.0, y: 50.0 },
        ];
        const EXPECTED: Vector = Vector { x: 3.0, y: 30.0 };

        // act
        let result: Vector = Vector::mean(input);

        // assert
        assert_eq!(result, EXPECTED);
    }
}
