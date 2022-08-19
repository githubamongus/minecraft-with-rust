use std::ops::DerefMut;

use camera::Camera;
use gl::*;
use glfw::{Context, Action, Window};
use glm::{Vec3, vec3, vec2};
use mesh::{Mesh, Vertex};

mod gl;
mod mesh;
mod camera;

pub const WINDOW_WIDTH: u32 = 1920;
pub const WINDOW_HEIGHT: u32 = 1080;

fn main() {
    //load the renderer dll to implement c++ code
    load_dll();

    //init glfw and set a few window hints
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::Decorated(false));
    glfw.window_hint(glfw::WindowHint::RefreshRate(Some(144)));

    //create window and make it the current context
    let (mut window, _) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Minecraft", glfw::WindowMode::Windowed).unwrap();
    window.make_current();

    load_gl();

    let vertices = [
        Vertex {
            position: vec3(-0.5, -0.5, -5.5),
            normal: vec3(0.0, 0.0, 0.0),
            uv: vec2(0.0, 0.0)
        },
        Vertex {
            position: vec3(-0.5, 0.5, -5.5),
            normal: vec3(0.0, 0.0, 0.0),
            uv: vec2(0.0, 1.0)
        },
        Vertex {
            position: vec3(0.5, 0.5, -5.5),
            normal: vec3(0.0, 0.0, 0.0),
            uv: vec2(1.0, 1.0)
        },
        Vertex {
            position: vec3(0.5, -0.5, -5.5),
            normal: vec3(0.0, 0.0, 0.0),
            uv: vec2(1.0, 0.0)
        }
    ];

    let mut test = Mesh::new("textures/test.png");
    test.set_vbo_data(&vertices);
    test.set_ebo_data(&[0, 2, 1, 0, 3, 2]);

    let mut camera = Camera::new(vec3(0.0, 0.0, 3.0));
    
    let mut delta_prev_time = std::time::Instant::now();
    //main loop
    while !window.should_close() {
        let delta_crnt_time = std::time::Instant::now();
        let delta_time = (delta_crnt_time - delta_prev_time).as_secs_f32();
        delta_prev_time = delta_crnt_time;
        
        glfw.poll_events();
        clear_screen();

        camera.inputs(&mut window, delta_time);

        //closes if escape is pressed
        if window.get_key(glfw::Key::Escape) == Action::Press {
            window.set_should_close(true);
        }
        
        test.draw(&camera);

        window.swap_buffers();
    }
}
