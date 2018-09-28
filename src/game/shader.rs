use std::ffi::{CStr, CString};
use std::ptr;
use std::str;

use super::gl;

use nalgebra::{Matrix4, Vector3};

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new() -> Shader {
        let mut shader = Shader { id: 0 };

        let vertex_string = String::from(
            "
            #version 330

            in vec2 position;

            out vec4 vColor;

            uniform mat4 uMVP;
            uniform vec3 uColor;

            void main() {
                vColor = vec4(uColor, 1.);
                gl_Position = uMVP * vec4(position, 0., 1.);
            }
        ",
        );
        let fragment_string = String::from(
            "
            #version 330

            in vec4 vColor;

            void main() {
                gl_FragColor = vColor;
            }
        ",
        );

        let vertex_code = CString::new(vertex_string.as_bytes()).unwrap();
        let fragment_code = CString::new(fragment_string.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vertex_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");
            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fragment_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");

            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;
        }

        shader
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }
    pub unsafe fn set_uniform_vec(&self, name: &CStr, value: &Vector3<f32>) {
        gl::Uniform3fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            value.as_slice().as_ptr(),
        );
    }
    pub unsafe fn set_uniform_mat(&self, name: &CStr, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_slice().as_ptr(),
        );
    }

    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as i32;
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1);

        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as i32 {
                // i8 is a GLchar
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut i8,
                );
                println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&info_log).unwrap());
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as i32 {
                gl::GetProgramInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut i8,
                );
                println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&info_log).unwrap());
            }
        }
    }
}
