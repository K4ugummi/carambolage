use super::glium;
use super::glium::index::IndexBuffer;
use super::glium::vertex::VertexBuffer;
use super::nalgebra::{Matrix4, Vector3};

use super::shader::generate_program;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub(super) struct Model {
    pub(super) color: Vector3<f32>,
    pub(super) vertex_buffer: VertexBuffer<Vertex>,
    pub(super) index_buffer: IndexBuffer<u16>,
    pub(super) matrix: Matrix4<f32>,

    pub(super) program: glium::Program,
}

impl Model {
    pub(super) fn new(color: Vector3<f32>, display: &glium::Display) -> Self {
        let (vertex_buffer, index_buffer) = get_test_buffer(display);
        Model {
            color,
            vertex_buffer,
            index_buffer,
            matrix: Matrix4::identity(),

            program: generate_program(display),
        }
    }
}

fn get_test_buffer(
    display: &glium::Display,
) -> (VertexBuffer<Vertex>, IndexBuffer<u16>) {
    let vertex_buffer = VertexBuffer::new(
        display,
        &[
            Vertex {
                position: [1., 1.5],
            },
            Vertex {
                position: [-1., 1.5],
            },
            Vertex {
                position: [-1., -1.5],
            },
            Vertex {
                position: [1., -1.5],
            },
        ],
    ).unwrap();

    let indices = [0u16, 1, 2, 0, 2, 3];
    let index_buffer = IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    ).unwrap();

    (vertex_buffer, index_buffer)
}
