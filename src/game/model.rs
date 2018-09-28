use std::ffi::CString;

use super::mesh::{Mesh, Vertex};
use super::shader::Shader;
use nalgebra::{Matrix4, Vector3};

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub shader: Shader,
}

impl Model {
    pub fn new() -> Model {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let vertices = vec![
            Vertex { position: [1., 1., 0.], },
            Vertex { position: [-1., 1., 0.], },
            Vertex { position: [-1., -1., 0.], },
            Vertex { position: [1., -1., 0.], },
        ];
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let indices = vec![
            0u32, 1, 2, 
            0, 2, 3,
        ];

        let mut meshes = Vec::new();

        meshes.push(Mesh::new(vertices, indices));

        let shader = Shader::new();

        Model { meshes, shader, }
    }

    pub fn draw(&self, mvp: &Matrix4<f32>) {
        unsafe {
            self.shader.use_program();
            self.shader.set_uniform_vec(
                &CString::new("uColor").unwrap(),
                &Vector3::new(1f32, 1., 1.),
            );
            self.shader.set_uniform_mat(
                &CString::new("uMVP").unwrap(),
                &mvp,
            );
            for mesh in &self.meshes {
                mesh.draw(&self.shader);
            }
        }
    }
}
