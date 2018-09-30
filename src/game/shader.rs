// This file is part of Carambolage.

// Carambolage is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Carambolage is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Carambolage.  If not, see <http://www.gnu.org/licenses/>.
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use super::gl;
use super::mesh::Texture;

use nalgebra::{Matrix4, Vector3};

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(file: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        // Load vertex shader code from file.
        let vertex_file_path = format!("res/shaders/{}.vs", file);
        let mut vertex_file = File::open(vertex_file_path)
            .unwrap_or_else(|_| panic!("ERROR: Failed to open {}.vs", file));
        let mut vertex_string = String::new();
        vertex_file
            .read_to_string(&mut vertex_string)
            .expect("ERROR: Failed to read vertex shader");
        let vertex_code = CString::new(vertex_string.as_bytes()).unwrap();

        // Load fragment shader code from file.
        let fragment_file_path = format!("res/shaders/{}.fs", file);
        let mut fragment_file = File::open(fragment_file_path)
            .unwrap_or_else(|_| panic!("ERROR: Failed to open {}.fs", file));
        let mut fragment_string = String::new();
        fragment_file
            .read_to_string(&mut fragment_string)
            .expect("ERROR: Failed to read fragment shader");
        let fragment_code = CString::new(fragment_string.as_bytes()).unwrap();

        // Try to compile both shaders.
        unsafe {
            // Compile vertex shader.
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vertex_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");

            // Compile fragment Shader.
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fragment_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");

            // Create program from vertex and fragment shader.
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;
        }

        shader
    }

    pub unsafe fn bind(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn bind_texture(&self, id: u32, tex: &Texture) {
        gl::ActiveTexture(gl::TEXTURE0 + id);
        gl::BindTexture(gl::TEXTURE_2D, tex.id);
    }

    pub unsafe fn _set_uniform_vec3(&self, name: &str, value: &Vector3<f32>) {
        let name_c = CString::new(name).unwrap();
        gl::Uniform3fv(
            gl::GetUniformLocation(self.id, name_c.as_ptr()),
            1,
            value.as_slice().as_ptr(),
        );
    }

    pub unsafe fn set_uniform_mat(&self, name: &str, mat: &Matrix4<f32>) {
        let name_c = CString::new(name).unwrap();
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name_c.as_ptr()),
            1,
            gl::FALSE,
            mat.as_slice().as_ptr(),
        );
    }

    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1);

        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                // i8 is a GLchar
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut i8,
                );
                println!(
                    "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n",
                    type_,
                    str::from_utf8(&info_log).unwrap_or("UNKNOWN")
                );
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                gl::GetProgramInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut i8,
                );
                println!(
                    "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n",
                    type_,
                    str::from_utf8(&info_log).unwrap_or("UNKNOWN")
                );
            }
        }
    }
}
