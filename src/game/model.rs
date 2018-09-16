use super::glium;
use super::glium::index::IndexBuffer;
use super::glium::index::NoIndices;
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
    pub(super) index_buffer: IndexBuffer<u32>,
    pub(super) matrix: Matrix4<f32>,

    pub(super) program: glium::Program,
}

fn get_test_buffer(
    display: &glium::Display,
) -> (VertexBuffer<Vertex>, IndexBuffer<u32>) {
    let vertex_buffer = //vec![0.2, 0.07, 0.2, -0.07, -0.2, -0.07, -0.2, 0.07];
        VertexBuffer::persistent(display, &[
            Vertex { position: [1., 1.], }, 
            Vertex { position: [-1., 1.], },
            Vertex { position: [-1., -1.], },
            Vertex { position: [1., -1.], }
        ]).unwrap();

    //let indices = [0u32, 1u32, 2u32, 1u32, 2u32, 3u32];
    let indices = [2u32, 1u32, 0u32, 3u32, 2u32, 1u32];
    let index_buffer = //vec![0, 1, 2, 1, 2, 3];
        //glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

    (vertex_buffer, index_buffer)
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
