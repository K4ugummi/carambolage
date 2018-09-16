use super::glium;

pub(super) fn generate_program(display: &glium::Display) -> glium::Program {
    let vertex_shader_src = r#"
        #version 330 core
        #extension GL_ARB_explicit_uniform_location : enable
        #extension GL_ARB_separate_shader_objects : enable

        layout(location = 0) in vec2 aPosition;

        layout(location = 0) out vec4 vColor;

        layout(location = 0) uniform mat4 uMVP;
        layout(location = 1) uniform vec3 uColor;

        void main() {
            vColor = vec4(uColor, 1.);
            gl_Position = uMVP * vec4(aPosition, 0., 1.);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330 core
        #extension GL_ARB_explicit_uniform_location : enable
        #extension GL_ARB_separate_shader_objects : enable

        layout(location = 0) in vec4 vColor;

        layout(location = 0) out vec4 fragColor;

        void main() {
            fragColor = vColor;
        }
    "#;

    let program = glium::Program::from_source(
        display,
        vertex_shader_src,
        fragment_shader_src,
        None,
    );

    assert!(program.is_ok());

    program.unwrap()
}

pub(super) struct Shader {}
