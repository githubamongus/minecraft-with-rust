use std::ops::DerefMut;

use gl::*;
use glfw::{Context, Action, Window};
use glm::{Vec3, vec3, vec2};
use mesh::{Mesh, Vertex};

mod gl;
mod mesh;

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

    let mut position = vec3(0.0, 0.0, 3.0);
    let mut direction = vec3(0.0, 0.0, -1.0);

    let proj = glm::perspective(WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32, 45.0f32.to_radians(), 0.1, 100.0);
    
    
    let mut delta_prev_time = std::time::Instant::now();
    //main loop
    while !window.should_close() {
        let delta_crnt_time = std::time::Instant::now();
        let delta_time = (delta_crnt_time - delta_prev_time).as_secs_f32();
        delta_prev_time = delta_crnt_time;
        
        glfw.poll_events();
        clear_screen();

        let view = glm::look_at(&position, &(position + direction), &glm::vec3(0.0, 1.0, 0.0));
        inputs(&mut position, &mut direction, &mut window, delta_time);

        //closes if escape is pressed
        if window.get_key(glfw::Key::Escape) == Action::Press {
            window.set_should_close(true);
        }
        
        test.draw(&proj, &view);

        window.swap_buffers();
    }
}

fn inputs(position: &mut Vec3, direction: &mut Vec3, window: &mut Window, delta_time: f32) {
    let up = vec3(0.0, 1.0f32, 0.0);

    let speed: f32 = 4.0;

    use glfw::Key::*;
    if window.is_focused() {
        let up = vec3(0.0, 1.0, 0.0f32);
        let speed: f32 = 4.0;
        let sensitivity: f64 = 120.0;

        use glfw::Action::*;
        use glfw::Key::*;
        if window.get_key(W) == Press {
            *position += speed * delta_time * *direction;
        }

        if window.get_key(S) == Press {
            *position -= speed * delta_time * *direction;
        }

        if window.get_key(A) == Press {
            *position -= speed * delta_time * glm::normalize(&glm::cross(direction, &up));
        }

        if window.get_key(D) == Press {
            *position += speed * delta_time * glm::normalize(&glm::cross(direction, &up));
        }

        if window.get_key(E) == Press {
            *position += speed * delta_time * up;
        }

        if window.get_key(Q) == Press {
            *position -= speed * delta_time * up;
        }

        window.set_cursor_mode(glfw::CursorMode::Hidden);

        let mut mouseX = 0.0;
        let mut mouseY = 0.0;
        (mouseX, mouseY) = window.get_cursor_pos();

        let rotX = sensitivity * ((mouseY - (WINDOW_HEIGHT / 2) as f64) / WINDOW_HEIGHT as f64);
        let rotY = sensitivity * ((mouseX - (WINDOW_WIDTH / 2) as f64) / WINDOW_WIDTH as f64);
        let newDirection = glm::rotate_vec3(&direction, (-rotX).to_radians() as f32, &glm::cross(&direction, &up));

        if newDirection.y <= 0.975 && newDirection.y >= -0.975 {
            *direction = newDirection;
        }
        *direction = glm::rotate_vec3(direction, (-rotY).to_radians() as f32, &up);

        window.set_cursor_pos((WINDOW_WIDTH / 2) as f64, (WINDOW_HEIGHT / 2) as f64);
    }
}
