use std::{fs::read_to_string, path::Path};

use cgmath::{perspective, vec3, vec4, Deg, EuclideanSpace, Matrix4, Point3};
use glfw::{Action, Key};
use learn_opengl::{
    camera::{Camera, CameraDirection, CameraDirectionTrait},
    gls::{
        buffers::{bindable::Bindable, texture::Texture2D, Attribute, VOs},
        shader::{Shader, ShaderProgram},
    },
    lights::{material::Material, point_lights::PointLightBuilder},
    window::Window,
};

use maze::logic::{Maze, MazeEntry, MazeIndex};
const SCR_WIDTH: u32 = 1600;
const SCR_HEIGHT: u32 = 1200;

const VERTEX_SHADER_SOURCE: &'static str = include_str!("../../shader/vert.glsl");
const FRAGMENT_SHADER_SOURCE: &'static str = include_str!("../../shader/frag.glsl");

const VERTEX_SHADER_SOURCE_LAMP: &'static str = include_str!("../../shader/lamp_vert.glsl");
const FRAGMENT_SHADER_SOURCE_LAMP: &'static str = include_str!("../../shader/lamp_frag.glsl");

fn main() {
    let mut maze: Maze = read_to_string("./maze.txt")
        .expect("Failed to find file maze.txt")
        .parse()
        .expect("Error parsing maze");
    let mut window = Window::new(SCR_WIDTH, SCR_HEIGHT, "Learn Opengl", false, false).unwrap();
    let v_shader =
        Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER).expect("Failed to Compile V Shader");
    let f_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)
        .expect("Failed to Compile F Shader");
    let shader = ShaderProgram::new([v_shader, f_shader]).expect("Failed to Create Shader Program");
    let lamp_v_shader = Shader::new(VERTEX_SHADER_SOURCE_LAMP, gl::VERTEX_SHADER)
        .expect("Failed to Compile V Shader");
    let lamp_f_shader = Shader::new(FRAGMENT_SHADER_SOURCE_LAMP, gl::FRAGMENT_SHADER)
        .expect("Failed to Compile F Shader");
    let lamp_shader = ShaderProgram::new([lamp_v_shader, lamp_f_shader])
        .expect("Failed to Create Shader Program");
    #[rustfmt::skip]
    let cube_verts: [f32; 288] = [
        // positions       // normals        // texture coords
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0
        ];
    let attributes = [
        Attribute {
            // cords
            location: 0,
            size: 3,
            normalized: false,
            stride: 8,
            offset: 0,
        },
        Attribute {
            // normal
            location: 1,
            size: 3,
            normalized: false,
            stride: 8,
            offset: 3,
        },
        Attribute {
            // texture
            location: 2,
            size: 2,
            normalized: false,
            stride: 8,
            offset: 6,
        },
    ];
    let vbo_vba =
        VOs::new(&cube_verts, &attributes, gl::TRIANGLES).expect("vbo or vba failed to bind");

    // textures
    let floor_textures = [
        Texture2D::new(
            image::open(&Path::new("./maze/resources/floor/albedo.jpg")).unwrap(),
            [gl::REPEAT, gl::REPEAT],
            [gl::LINEAR, gl::LINEAR],
            gl::RGB,
            None,
        )
        .unwrap(),
        Texture2D::new(
            image::open(&Path::new("./maze/resources/floor/specular.jpg")).unwrap(),
            [gl::REPEAT, gl::REPEAT],
            [gl::LINEAR, gl::LINEAR],
            gl::RGB,
            None,
        )
        .unwrap(),
    ];

    let floor_texture =
        learn_opengl::gls::buffers::texture::Textures::<2>::new(core::array::from_fn(|i| {
            &floor_textures[i]
        }))
        .unwrap();
    floor_texture.bind().unwrap();

    shader
        .set_uniform("material", Material::new(0, 1, 0.5))
        .expect("error with material");

    let floor_light = PointLightBuilder::default()
        .pos(maze.get_player_loc() + vec3(0., 0., 0.))
        .build();

    shader
        .set_uniform("pointLight", &floor_light)
        .expect("couldn't send point light uniform");

    let mut cam = Camera::new(
        Point3::from_vec(maze.get_player_loc()),
        90f32,
        0f32,
        vec3(2.5, 2.5, 2.5),
    );

    let mut projection: Matrix4<f32> =
        perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
    shader.set_uniform("projection", projection).unwrap();
    lamp_shader.set_uniform("projection", projection).unwrap();

    window.app_loop(|mut w| {
        shader.use_program();
        shader.set_uniform("viewPos", cam.get_pos()).unwrap();
        process_events(&mut w, &mut cam, &mut projection);
        let dir = process_input(&mut w.window);
        if let Some(dir) = dir {
            if dir != 0 {
                let time = w.delta_time;
                let new_pos: MazeIndex = cam.try_translate_camera(dir, time).into();
                if maze[new_pos] != MazeEntry::Wall {
                    cam.translate_camera(dir, time);
                    maze.move_player_to(new_pos);
                }
            }
        }
        let view = cam.get_view();
        shader.set_uniform("view", view.clone()).unwrap();

        for (y, row) in maze.iter().enumerate() {
            for (x, entry) in row.iter().enumerate() {
                let model: Matrix4<f32> = match entry {
                    maze::logic::MazeEntry::Wall => {
                        shader
                            .set_uniform("uColor", vec4(1.0, 0f32, 0., 1.))
                            .unwrap();
                        Matrix4::from_translation(vec3(x as f32, 1., y as f32))
                    }
                    maze::logic::MazeEntry::Empty => {
                        shader
                            .set_uniform("uColor", vec4(1.0, 1f32, 1., 1.))
                            .unwrap();
                        Matrix4::from_translation(vec3(x as f32, 0., y as f32))
                    }
                    maze::logic::MazeEntry::Start => {
                        shader
                            .set_uniform("uColor", vec4(0.0, 1f32, 0., 1.))
                            .unwrap();
                        Matrix4::from_translation(vec3(x as f32, 0., y as f32))
                    }
                    maze::logic::MazeEntry::End => {
                        shader
                            .set_uniform("uColor", vec4(1.0, 1f32, 0., 1.))
                            .unwrap();
                        Matrix4::from_translation(vec3(x as f32, 0., y as f32))
                    }
                };
                shader.set_uniform("model", model).unwrap();
                vbo_vba.draw_arrays(0, 36).unwrap();
            }
            lamp_shader.use_program();
            lamp_shader.set_uniform("view", view).unwrap();
            lamp_shader
                .set_uniform(
                    "model",
                    Matrix4::from_translation(floor_light.get_pos()) * Matrix4::from_scale(0.2),
                )
                .unwrap();
            vbo_vba.draw_arrays(0, 36).unwrap();
        }
    });
}

fn process_events(w: &mut Window, cam: &mut Camera, proj: &mut Matrix4<f32>) -> bool {
    for (_, event) in glfw::flush_messages(&w.events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                w.width = width as u32;
                w.height = height as u32;
                *proj = perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
                unsafe {
                    gl::Viewport(0, 0, width, height);
                };
            }
            glfw::WindowEvent::CursorPos(x, y) => {
                cam.move_point_pos(x as f32, y as f32);
            }
            _ => {}
        };
    }
    return false;
}
fn process_input(window: &mut glfw::Window) -> Option<CameraDirection> {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
        return None;
    }
    let mut dirs = CameraDirection::new();

    if window.get_key(Key::W) == Action::Press {
        dirs.toggle_forward();
    }

    if window.get_key(Key::S) == Action::Press {
        dirs.toggle_backward();
    }

    if window.get_key(Key::D) == Action::Press {
        dirs.toggle_right();
    }

    if window.get_key(Key::A) == Action::Press {
        dirs.toggle_left();
    }

    // if window.get_key(Key::LeftShift) == Action::Press {
    //     dirs.toggle_down();
    // }
    //
    // if window.get_key(Key::Space) == Action::Press {
    //     dirs.toggle_up();
    // }
    return Some(dirs);
}
