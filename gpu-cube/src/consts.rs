use std::{fmt::Display, ops::Mul};

use cube::algorithms::Move;

pub const GREEN: [f32; 3] = [0.0, 1.0, 0.0];
pub const BLUE: [f32; 3] = [0.0, 0.0, 1.0];
pub const ORANGE: [f32; 3] = [0.8, 0.4, 0.0];
pub const RED: [f32; 3] = [1.0, 0.0, 0.0];
pub const WHITE: [f32; 3] = [0.8, 0.8, 0.8];
pub const YELLOW: [f32; 3] = [1.0, 1.0, 0.0];

pub const BLACK: [f32; 3] = [0.2, 0.2, 0.2];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Mul<Direction> for f32 {
    type Output = f32;

    fn mul(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::Clockwise => self,
            Direction::CounterClockwise => -self,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Faces {
    Front,
    Back,
    Left,
    Right,
    Up,
    Down,
}

impl Faces {
    pub fn color(self) -> [f32; 3] {
        match self {
            Faces::Front => GREEN,
            Faces::Back => BLUE,
            Faces::Left => ORANGE,
            Faces::Right => RED,
            Faces::Up => WHITE,
            Faces::Down => YELLOW,
        }
    }

    pub fn get_transformation(self, theta: f32) -> cgmath::Matrix4<f32> {
        match self {
            Faces::Front => cgmath::Matrix4::from_angle_z(cgmath::Rad(-theta)),
            Faces::Back => cgmath::Matrix4::from_angle_z(cgmath::Rad(theta)),
            Faces::Left => cgmath::Matrix4::from_angle_x(cgmath::Rad(theta)),
            Faces::Right => cgmath::Matrix4::from_angle_x(cgmath::Rad(-theta)),
            Faces::Up => cgmath::Matrix4::from_angle_y(cgmath::Rad(-theta)),
            Faces::Down => cgmath::Matrix4::from_angle_y(cgmath::Rad(theta)),
        }
    }

    pub fn to_move(self, direction: Direction) -> Move {
        match (self, direction) {
            (Faces::Front, Direction::Clockwise) => Move::F,
            (Faces::Front, Direction::CounterClockwise) => Move::Fp,
            (Faces::Back, Direction::Clockwise) => Move::B,
            (Faces::Back, Direction::CounterClockwise) => Move::Bp,
            (Faces::Left, Direction::Clockwise) => Move::L,
            (Faces::Left, Direction::CounterClockwise) => Move::Lp,
            (Faces::Right, Direction::Clockwise) => Move::R,
            (Faces::Right, Direction::CounterClockwise) => Move::Rp,
            (Faces::Up, Direction::Clockwise) => Move::U,
            (Faces::Up, Direction::CounterClockwise) => Move::Up,
            (Faces::Down, Direction::Clockwise) => Move::D,
            (Faces::Down, Direction::CounterClockwise) => Move::Dp,
        }
    }
}

impl Display for Faces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_str = match self {
            Faces::Front => "Front",
            Faces::Back => "Back",
            Faces::Left => "Left",
            Faces::Right => "Right",
            Faces::Up => "Top",
            Faces::Down => "Bottom",
        };
        f.write_str(self_str)
    }
}
