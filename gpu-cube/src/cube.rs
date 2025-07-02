use std::{f32::consts::PI, time::Duration, vec};

use cube::algorithms::Move;

use crate::{
    consts::{Direction, Faces},
    cubie::Cubie,
};

pub struct Cube {
    animation_speed: f32,          // Radians per second
    animation_progress: f32,       // Progress of the animation from 0.0 to 1.0
    animation_face: Option<Faces>, // The face turned during the animation
    pub cubies: Vec<Cubie>,
    animation_direction: Direction,
    move_queue: Vec<(Faces, Direction)>,
    pub cube: cube::Cube, // Non-graphic representation of the cube
}

impl Cube {
    pub fn new(device: &wgpu::Device) -> Self {
        let mut cubies = Vec::new();

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let mut faces = vec![];
                    if x == 1 {
                        faces.push(Faces::Right);
                    } else if x == -1 {
                        faces.push(Faces::Left);
                    }
                    if y == 1 {
                        faces.push(Faces::Up);
                    } else if y == -1 {
                        faces.push(Faces::Down);
                    }
                    if z == 1 {
                        faces.push(Faces::Front);
                    } else if z == -1 {
                        faces.push(Faces::Back);
                    }

                    let offset =
                        cgmath::Vector3::new((2 * x) as f32, (2 * y) as f32, (2 * z) as f32);
                    let transformation = cgmath::Matrix4::from_translation(offset);

                    cubies.push(Cubie::new(device, faces, transformation));
                }
            }
        }

        Cube {
            animation_speed: PI, // 1 half turn per second
            animation_progress: 0.0,
            animation_face: None,
            animation_direction: Direction::Clockwise,
            cubies,
            move_queue: Vec::new(),
            cube: cube::Cube::default(),
        }
    }

    pub fn start_animation(&mut self, face: Faces, direction: Direction) {
        if !self.animation_face.is_some() {
            self.cube.execute_move(&face.to_move(&direction));
            self.animation_face = Some(face);
            self.animation_progress = 0.0;
            self.animation_direction = direction;
        }
    }

    pub fn update_cubies(&mut self, delta_time: Duration, queue: &wgpu::Queue) {
        if let Some(animation_face) = self.animation_face {
            self.animation_progress +=
                (delta_time.as_secs_f32() * self.animation_speed) / (PI / 2.0);

            if self.animation_progress >= 1.0 {
                self.round_positions(&queue);

                self.animation_progress = 0.0;
                // TODO: Clamp the positions of the cubies to ensure they don't slightly change over time
                self.animation_face = None;

                if let Some((face, direction)) = self.move_queue.pop() {
                    self.start_animation(face, direction);
                }
                return;
            }
            let theta = delta_time.as_secs_f32()
                * self.animation_speed
                * f32::from(&self.animation_direction);

            self.cubies.iter_mut().for_each(|cubie| {
                if cubie.is_currently_in_face(animation_face) {
                    cubie.transform(animation_face.get_transformation(theta), queue);
                }
            });
        }
    }

    fn round_positions(&mut self, queue: &wgpu::Queue) {
        self.cubies.iter_mut().for_each(|cubie| {
            cubie.round_positions(&queue);
        });
    }

    pub fn start_algorithm(&mut self, algorithm: &[Move]) {
        if !self.move_queue.is_empty() || algorithm.is_empty() {
            return;
        }

        let queue = algorithm
            .iter()
            .flat_map(|move_| match move_ {
                Move::U => vec![(Faces::Up, Direction::Clockwise)],
                Move::D => vec![(Faces::Down, Direction::Clockwise)],
                Move::F => vec![(Faces::Front, Direction::Clockwise)],
                Move::B => vec![(Faces::Back, Direction::Clockwise)],
                Move::R => vec![(Faces::Right, Direction::Clockwise)],
                Move::L => vec![(Faces::Left, Direction::Clockwise)],
                Move::U2 => vec![
                    (Faces::Up, Direction::Clockwise),
                    (Faces::Up, Direction::Clockwise),
                ],
                Move::D2 => vec![
                    (Faces::Down, Direction::Clockwise),
                    (Faces::Down, Direction::Clockwise),
                ],
                Move::F2 => vec![
                    (Faces::Front, Direction::Clockwise),
                    (Faces::Front, Direction::Clockwise),
                ],
                Move::B2 => vec![
                    (Faces::Back, Direction::Clockwise),
                    (Faces::Back, Direction::Clockwise),
                ],
                Move::R2 => vec![
                    (Faces::Right, Direction::Clockwise),
                    (Faces::Right, Direction::Clockwise),
                ],
                Move::L2 => vec![
                    (Faces::Left, Direction::Clockwise),
                    (Faces::Left, Direction::Clockwise),
                ],
                Move::Up => vec![(Faces::Up, Direction::CounterClockwise)],
                Move::Dp => vec![(Faces::Down, Direction::CounterClockwise)],
                Move::Fp => vec![(Faces::Front, Direction::CounterClockwise)],
                Move::Bp => vec![(Faces::Back, Direction::CounterClockwise)],
                Move::Rp => vec![(Faces::Right, Direction::CounterClockwise)],
                Move::Lp => vec![(Faces::Left, Direction::CounterClockwise)],
                Move::None => vec![],
            })
            .rev()
            .collect::<Vec<_>>();

        self.move_queue.extend(queue);

        if let Some((face, direction)) = self.move_queue.pop() {
            self.start_animation(face, direction);
        }
    }
}
