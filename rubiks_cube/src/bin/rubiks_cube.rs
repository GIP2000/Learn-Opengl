use cgmath::{perspective, Deg, Matrix4, Point3, Vector4};
use glfw::{Action, Key};
use learn_opengl::{
    gls::{
        buffers::{Attribute, VOs},
        shader::{Shader, ShaderProgram},
    },
    window::Window,
};
use rubiks_cube::{camera::Camera, game_logic::RubiksCube};

const SCR_WIDTH: u32 = 1600;
const SCR_HEIGHT: u32 = 1200;

const VERTEX_SHADER_SOURCE: &'static str = include_str!("../../shaders/vert.glsl");
const FRAG_SHADER_SOURCE: &'static str = include_str!("../../shaders/frag.glsl");

fn main() {
    let mut window = Window::new(SCR_WIDTH, SCR_HEIGHT, "Rubiks Cube").unwrap();
    let v_shader =
        Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER).expect("Failed to compile V Shader");
    let f_shader =
        Shader::new(FRAG_SHADER_SOURCE, gl::FRAGMENT_SHADER).expect("Failed to compile F Shader");
    let shader = ShaderProgram::new([v_shader, f_shader]).expect("Failed to Create Shader Program");
    let face_verts: [f32; 18] = [
        -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5,
        0.5,
    ];

    let attributes = [Attribute {
        // cords
        location: 0,
        size: 3,
        normalized: false,
        stride: 3,
        offset: 0,
    }];
    let cube =
        VOs::new(&face_verts, &attributes, gl::TRIANGLES).expect("vbo or vba failed to bind");

    let mut projection: Matrix4<f32> =
        perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
    shader.set_uniform("projection", projection).unwrap();
    let mut cam = Camera::new(Point3::new(1., 1., 10.), Point3::new(1., 1., 1.));

    let mut cube_state = RubiksCube::new();
    let mut last_left = false;
    window.app_loop(|mut w| {
        let (three_clicked, is_left_click, is_right_click) = process_input(&w.window);
        process_events(&mut w, &mut projection, &mut cam, is_left_click, last_left);
        last_left = is_left_click;

        shader.set_uniform("view", cam.get_view()).unwrap();

        if three_clicked {
            cube_state.rotate(8, true).unwrap();
        }

        for (i, block) in cube_state.iter().enumerate() {
            for (y, row) in block.iter().enumerate() {
                for (x, color) in row.iter().enumerate() {
                    let model = block.convert_cords(x as f32, y as f32) * block.get_rotation();
                    shader.set_uniform("model", model).unwrap();
                    shader
                        .set_uniform::<Vector4<f32>>("uColor", color.into())
                        .unwrap();
                    cube.draw_arrays(0, 6).unwrap();
                }
            }
        }
    });
}
fn process_input(window: &glfw::Window) -> (bool, bool, bool) {
    (
        window.get_key(Key::Num3) == Action::Press,
        window.get_mouse_button(glfw::MouseButton::Button1) == Action::Press,
        window.get_mouse_button(glfw::MouseButton::Button2) == Action::Press,
    )
}

fn process_events(
    w: &mut Window,
    proj: &mut Matrix4<f32>,
    cam: &mut Camera,
    is_left_click: bool,
    last_left: bool,
) -> bool {
    glfw::flush_messages(&w.events)
        .into_iter()
        .for_each(|(_, event)| {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    w.width = width as u32;
                    w.height = height as u32;
                    *proj =
                        perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    };
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    if is_left_click && last_left {
                        cam.pan(x as f32, y as f32, w.delta_time)
                    } else if is_left_click && !last_left {
                        cam.set_last(x as f32, y as f32);
                    }
                }
                _ => {}
            };
        });
    return false;
}
