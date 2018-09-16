use super::glium;
use super::nalgebra::{Matrix4, Vector3};

use super::shader::generate_program;

pub(super) struct Model {
    pub(super) color: Vector3<f32>,
    vertex_buffer: Vec<f32>,
    index_buffer: Vec<usize>,
    matrix: Matrix4<f32>,

    program: glium::Program,
}

fn get_test_buffer() -> (Vec<f32>, Vec<usize>) {
    let vertex_buffer = vec![0.2, 0.07, 0.2, -0.07, -0.2, -0.07, -0.2, 0.07];

    let index_buffer = vec![0, 3, 2, 2, 1, 3];

    (vertex_buffer, index_buffer)
}

impl Model {
    pub(super) fn new(color: Vector3<f32>, display: &glium::Display) -> Self {
        let (vertex_buffer, index_buffer) = get_test_buffer();
        Model {
            color,
            vertex_buffer,
            index_buffer,
            matrix: Matrix4::identity(),

            program: generate_program(display),
        }
    }
}
