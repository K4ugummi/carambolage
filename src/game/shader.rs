use super::glium;

const VERTEX_SHADER_SRC: &str = "
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

const FRAGMENT_SHADER_SRC: &str = "
        #version 330

        in vec4 vColor;

        out vec4 fragColor;

        void main() {
            fragColor = vColor;
        }
    ";

pub(super) fn generate_program(display: &glium::Display) -> glium::Program {
    let program = glium::Program::from_source(
        display,
        VERTEX_SHADER_SRC,
        FRAGMENT_SHADER_SRC,
        None,
    );
    program.unwrap()
}

#[cfg(test)]
mod test {
    use super::{FRAGMENT_SHADER_SRC, VERTEX_SHADER_SRC};
    use game::glium;
    use glium::glutin;

    #[test]
    fn compile_shader() {
        let version = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3));
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new().with_visibility(false);
        let context = glutin::ContextBuilder::new()
            .with_gl_debug_flag(true)
            .with_gl(version);

        let display =
            glium::Display::new(window, context, &events_loop).unwrap();

        let program = glium::Program::from_source(
            &display,
            VERTEX_SHADER_SRC,
            FRAGMENT_SHADER_SRC,
            None,
        );

        assert!(program.is_ok());
    }
}
