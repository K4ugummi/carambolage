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
use crate::grphx::Texture;
use gl;
use log::{debug, error};
use nalgebra::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

/// Compiled GLSL Shader Program.
pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(file: &str) -> Shader {
        debug!("New {}", file);
        let mut shader = Shader { id: 0 };

        // Load vertex shader code from file.
        let vertex_file_path = format!("res/shaders/{}.vs", file);
        let mut vertex_file = File::open(vertex_file_path).unwrap_or_else(|_| {
            error!("Failed to open {}.vs", file);
            panic!()
        });
        let mut vertex_string = String::new();
        vertex_file.read_to_string(&mut vertex_string).unwrap_or_else(|_| {
            error!("Failed to read vertex shader");
            panic!()
        });
        let vertex_code = CString::new(vertex_string.as_bytes()).unwrap();

        // Load fragment shader code from file.
        let fragment_file_path = format!("res/shaders/{}.fs", file);
        let mut fragment_file = File::open(fragment_file_path).unwrap_or_else(|_| {
            error!("Failed to open {}.fs", file);
            panic!()
        });
        let mut fragment_string = String::new();
        fragment_file.read_to_string(&mut fragment_string).unwrap_or_else(|_| {
            error!("Failed to read fragment shader");
            panic!()
        });
        let fragment_code = CString::new(fragment_string.as_bytes()).unwrap();

        // Try to compile both shaders.
        unsafe {
            // Compile vertex shader.
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vertex_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VertexShader");

            // Compile fragment Shader.
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fragment_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FragmentShader");

            // Create program from vertex and fragment shader.
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "ShaderProgram");

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;
        }

        shader
    }

    /// Bind the shader program.
    pub unsafe fn bind(&self) {
        gl::UseProgram(self.id);
    }

    /// Return the location of the uniform by its name for the currently bound shader.
    pub unsafe fn _get_uniform_location(&self, name: &str) -> i32 {
        let cstr = CString::new(name).unwrap();
        gl::GetUniformLocation(self.id, cstr.as_ptr())
    }

    /// Bind a `Texture` to the currently bound shader program at location `id`.
    pub unsafe fn bind_texture(id: u32, tex: &Texture) {
        gl::ActiveTexture(gl::TEXTURE0 + id);
        gl::BindTexture(gl::TEXTURE_2D, tex.id);
    }

    /// Bind a `Vector2<f32>` to the currently boundshader program at location `id`.
    pub unsafe fn _set_uniform_vec2(id: i32, value: Vector2<f32>) {
        gl::Uniform2fv(id, 1, value.as_slice().as_ptr());
    }

    /// Bind a `Vector3<f32>` to the currently boundshader program at location `id`.
    pub unsafe fn _set_uniform_vec3(id: i32, value: &Vector3<f32>) {
        gl::Uniform3fv(id, 1, value.as_slice().as_ptr());
    }

    /// Bind a `Vector4<f32>` to the currently boundshader program at location `id`.
    pub unsafe fn _set_uniform_vec4(id: i32, value: &Vector4<f32>) {
        gl::Uniform4fv(id, 1, value.as_slice().as_ptr());
    }

    /// Bind a `Matrix2<f32>` to the currently boundshader program at location `id`.
    pub unsafe fn _set_uniform_mat2(id: i32, mat: &Matrix2<f32>) {
        gl::UniformMatrix2fv(id, 1, gl::FALSE, mat.as_slice().as_ptr());
    }

    /// Bind a `Matrix3<f32>` to the currently boundshader program at location `id`.
    pub unsafe fn _set_uniform_mat3(id: i32, mat: &Matrix3<f32>) {
        gl::UniformMatrix3fv(id, 1, gl::FALSE, mat.as_slice().as_ptr());
    }

    /// Bind a `Matrix4<f32>` to the currently bound shader program at location `id`.
    pub unsafe fn set_uniform_mat4(id: i32, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(id, 1, gl::FALSE, mat.as_slice().as_ptr());
    }

    pub unsafe fn _set_uniform_int(id: i32, value: i32) {
        gl::Uniform1i(id, value);
    }

    ///
    unsafe fn check_compile_errors(&self, shader: u32, shader_type: &str) {
        debug!("Checking {} shader for compile errors", shader_type);
        let mut success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1);

        if shader_type != "ShaderProgram" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                // i8 is a GLchar
                gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);
                error!(
                    "Compilation error of type: {}\nInfo log:\n{}",
                    shader_type,
                    str::from_utf8(&info_log).unwrap_or("UNKNOWN")
                );
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);
                error!(
                    "Linking error of type: {}\nInfo log:\n{}",
                    shader_type,
                    str::from_utf8(&info_log).unwrap_or("UNKNOWN")
                );
            }
        }
    }
}
