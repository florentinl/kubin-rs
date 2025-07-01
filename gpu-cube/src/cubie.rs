use cgmath::Transform as _;
use wgpu::util::DeviceExt;

use crate::{
    consts::{BLACK, Faces},
    vertex::Vertex,
};

pub struct CubieFace {
    triangles: [[usize; 3]; 2],
    face: Faces,
}

pub struct Cubie {
    pub position: [f32; 3],
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    vertices: Vec<Vertex>,
    pub faces: Vec<Faces>,
}

impl Cubie {
    const VERTEX_POSITIONS: [[f32; 3]; 8] = [
        // Back, Left, Down
        [-1.0, -1.0, -1.0],
        // Back, Right, Down
        [1.0, -1.0, -1.0],
        // Back, Right, Up
        [1.0, 1.0, -1.0],
        // Back, Left, Up
        [-1.0, 1.0, -1.0],
        // Front, Left, Down
        [-1.0, -1.0, 1.0],
        // Front, Right, Down
        [1.0, -1.0, 1.0],
        // Front, Right, Up
        [1.0, 1.0, 1.0],
        // Front, Left, Up
        [-1.0, 1.0, 1.0],
    ];

    const FACES: [CubieFace; 6] = [
        // Front face
        CubieFace {
            triangles: [[4, 6, 7], [4, 5, 6]],
            face: Faces::Front,
        },
        // Back face
        CubieFace {
            triangles: [[0, 3, 1], [3, 2, 1]],
            face: Faces::Back,
        },
        // Left face
        CubieFace {
            triangles: [[4, 3, 0], [4, 7, 3]],
            face: Faces::Left,
        },
        // Right face
        CubieFace {
            triangles: [[1, 2, 5], [2, 6, 5]],
            face: Faces::Right,
        },
        // Top face
        CubieFace {
            triangles: [[3, 7, 2], [2, 7, 6]],
            face: Faces::Up,
        },
        // Bottom face
        CubieFace {
            triangles: [[0, 1, 4], [1, 5, 4]],
            face: Faces::Down,
        },
    ];

    pub fn new(
        device: &wgpu::Device,
        faces: Vec<Faces>,
        transformation: cgmath::Matrix4<f32>,
    ) -> Self {
        let mut vertices = Vec::new();
        for face in Self::FACES.iter() {
            for triange in face.triangles.iter() {
                for &index in triange {
                    let position = Self::VERTEX_POSITIONS[index];
                    let position: [f32; 3] = transformation
                        .transform_point(cgmath::Point3::from(position))
                        .into();

                    let color = if faces.contains(&face.face) {
                        face.face.color()
                    } else {
                        BLACK
                    };
                    vertices.push(Vertex { position, color });
                }
            }
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Cubie Vertex Buffer",)),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let vertex_count = vertices.len() as u32;

        let position = transformation
            .transform_point(cgmath::Point3::from([0.0, 0.0, 0.0]))
            .into();

        Cubie {
            position,
            vertices,
            vertex_buffer,
            vertex_count,
            faces,
        }
    }

    pub fn is_currently_in_face(&self, face: Faces) -> bool {
        match face {
            Faces::Front => self.position[2] > 1.,
            Faces::Back => self.position[2] < -1.,
            Faces::Left => self.position[0] < -1.,
            Faces::Right => self.position[0] > 1.,
            Faces::Up => self.position[1] > 1.,
            Faces::Down => self.position[1] < -1.,
        }
    }

    pub fn transform(&mut self, transformation: cgmath::Matrix4<f32>, queue: &wgpu::Queue) {
        for vertex in self.vertices.iter_mut() {
            let position: [f32; 3] = transformation
                .transform_point(cgmath::Point3::from(vertex.position))
                .into();
            vertex.position = position;
        }

        self.position = transformation
            .transform_point(cgmath::Point3::from(self.position))
            .into();

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
    }

    pub fn round_positions(&mut self, queue: &wgpu::Queue) {
        self.position = [
            self.position[0].round(),
            self.position[1].round(),
            self.position[2].round(),
        ];
        for vertex in self.vertices.iter_mut() {
            vertex.position = [
                vertex.position[0].round(),
                vertex.position[1].round(),
                vertex.position[2].round(),
            ];
        }

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
    }
}
