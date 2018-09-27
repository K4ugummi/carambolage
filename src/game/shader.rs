use super::glium;

pub(super) fn generate_program(display: &glium::Display) -> glium::Program {
    let vertex_shader_src = "
        #version 330

        in vec2 position;

        out vec4 vColor;

        uniform mat4 uMVP;
        uniform vec3 uColor;

        void main() {
            vColor = vec4(uColor, 1.);
            gl_Position = uMVP * vec4(position, 0., 1.);
        }
    ";

    let fragment_shader_src = "
        #version 330

        in vec4 vColor;

        out vec4 fragColor;

        void main() {
            fragColor = vColor;
        }
    ";

    let program = glium::Program::from_source(
        display,
        vertex_shader_src,
        fragment_shader_src,
        None,
    );

    assert!(program.is_ok());

    program.unwrap()
}
