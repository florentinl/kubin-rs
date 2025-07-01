use std::{f32::consts::PI, time::Duration, vec};

use cube::algorithms::Move;
use cube::corner::Corner;
use cube::edge::Edge;
use cube::*;

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
        }
    }

    pub fn start_animation(&mut self, face: Faces, direction: Direction) {
        if !self.animation_face.is_some() {
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

    pub fn to_non_graphic_cube(&self) -> cube::Cube {
        let mut result = cube::Cube::default();

        // Map each cubie position to the corresponding edge or corner
        for cubie in &self.cubies {
            if cubie.faces.len() == 2 {
                let piece_from_faces = self.edge_piece_from_faces(&cubie.faces);
                let piece_from_position = self.edge_piece_from_position(cubie.position);

                result.edges[piece_from_position] = Edge {
                    piece: piece_from_faces,
                    orientation: 0, // Default orientation
                };
            } else if cubie.faces.len() == 3 {
                // Handle corner pieces
                let piece_from_faces = self.corner_piece_from_faces(&cubie.faces);
                let piece_from_position = self.corner_piece_from_position(cubie.position);

                result.corners[piece_from_position] = Corner {
                    piece: piece_from_faces,
                    orientation: 0, // Default orientation
                };
            }
        }

        result
    }

    fn corner_piece_from_faces(&self, faces: &[Faces]) -> cube::corner::Piece {
        // Convert the faces to a corner piece
        let mut up = false;
        let mut down = false;
        let mut left = false;
        let mut right = false;
        let mut front = false;
        let mut back = false;
        for face in faces {
            match face {
                Faces::Up => up = true,
                Faces::Down => down = true,
                Faces::Left => left = true,
                Faces::Right => right = true,
                Faces::Front => front = true,
                Faces::Back => back = true,
            }
        }

        if up && front && left {
            cube::corner::Piece::Ufl
        } else if up && right && front {
            cube::corner::Piece::Urf
        } else if up && back && right {
            cube::corner::Piece::Ubr
        } else if up && left && back {
            cube::corner::Piece::Ulb
        } else if down && left && front {
            cube::corner::Piece::Dlf
        } else if down && front && right {
            cube::corner::Piece::Dfr
        } else if down && right && back {
            cube::corner::Piece::Drb
        } else if down && back && left {
            cube::corner::Piece::Dbl
        } else {
            panic!("Invalid corner piece configuration")
        }
    }

    fn corner_piece_from_position(&self, position: [f32; 3]) -> usize {
        // Convert the position to a corner piece
        let scale = 2.0;
        let x = (position[0] / scale).round() as i32;
        let y = (position[1] / scale).round() as i32;
        let z = (position[2] / scale).round() as i32;

        match (x, y, z) {
            (1, 1, 1) => URF,
            (1, 1, -1) => UBR,
            (1, -1, 1) => DFR,
            (1, -1, -1) => DRB,
            (-1, 1, 1) => UFL,
            (-1, 1, -1) => ULB,
            (-1, -1, 1) => DLF,
            (-1, -1, -1) => DBL,
            _ => panic!("Invalid corner piece position"),
        }
    }

    fn edge_piece_from_faces(&self, faces: &[Faces]) -> cube::edge::Piece {
        // Convert the faces to an edge piece
        let mut up = false;
        let mut down = false;
        let mut left = false;
        let mut right = false;
        let mut front = false;
        let mut back = false;

        for face in faces {
            match face {
                Faces::Up => up = true,
                Faces::Down => down = true,
                Faces::Left => left = true,
                Faces::Right => right = true,
                Faces::Front => front = true,
                Faces::Back => back = true,
            }
        }

        if up && front {
            cube::edge::Piece::UF
        } else if up && right {
            cube::edge::Piece::UR
        } else if up && back {
            cube::edge::Piece::UB
        } else if up && left {
            cube::edge::Piece::UL
        } else if down && front {
            cube::edge::Piece::DF
        } else if down && right {
            cube::edge::Piece::DR
        } else if down && back {
            cube::edge::Piece::DB
        } else if down && left {
            cube::edge::Piece::DL
        } else if front && right {
            cube::edge::Piece::FR
        } else if right && back {
            cube::edge::Piece::BR
        } else if back && left {
            cube::edge::Piece::BL
        } else if left && front {
            cube::edge::Piece::FL
        } else {
            panic!("Invalid edge piece configuration")
        }
    }

    fn edge_piece_from_position(&self, position: [f32; 3]) -> usize {
        let scale = 2.0;
        // Convert the position to an edge piece
        let x = (position[0] / scale).round() as i32;
        let y = (position[1] / scale).round() as i32;
        let z = (position[2] / scale).round() as i32;

        match (x, y, z) {
            (1, 1, 0) => UR,
            (1, 0, 1) => FR,
            (1, 0, -1) => BR,
            (1, -1, 0) => DR,
            (-1, 1, 0) => UL,
            (-1, 0, 1) => FL,
            (-1, 0, -1) => BL,
            (-1, -1, 0) => DL,
            (0, 1, 1) => UF,
            (0, 1, -1) => UB,
            (0, -1, 1) => DF,
            (0, -1, -1) => DB,
            _ => panic!("Invalid edge piece position"),
        }
    }
}
