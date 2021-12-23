use crate::vector::Vector;
use std::convert::TryInto;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn mean(points: Vec<Point>) -> Point {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        for i in 0..points.len() {
            sum_x += points[i].x;
            sum_y += points[i].y;
        }

        let total: u32 = points.len().try_into().unwrap();
        Point {
            x: sum_x / total as f32,
            y: sum_y / total as f32,
        }
    }

    pub fn x(self) -> f32 {
        self.x
    }

    pub fn y(self) -> f32 {
        self.y
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

    pub fn vector_to(&self, other: &Point) -> Vector {
        Vector {
            dx: other.x - self.x,
            dy: other.y - self.y,
        }
    }

    pub fn move_forward(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_five_points_returns_mean() {
        // arrange
        let input: Vec<Point> = vec![
            Point { x: 1.0, y: 10.0 },
            Point { x: 2.0, y: 20.0 },
            Point { x: 3.0, y: 30.0 },
            Point { x: 4.0, y: 40.0 },
            Point { x: 5.0, y: 50.0 },
        ];
        const EXPECTED: Point = Point { x: 3.0, y: 30.0 };

        // act
        let result: Point = Point::mean(input);

        // assert
        assert_eq!(result, EXPECTED);
    }
}
