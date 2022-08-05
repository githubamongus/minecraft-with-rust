#![allow(non_upper_case_globals)]

mod gl;
mod drawable;
mod minecraft;

use drawable::Drawable;
use gl::*;
use glfw::{FAIL_ON_ERRORS, Context, Action, Window};
use glm::*;

const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2]
}

fn main() {
    let mut glfw = glfw::init(FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::Decorated(false));
    glfw.window_hint(glfw::WindowHint::RefreshRate(Some(144)));
    
    let (mut window, _event) = glfw.create_window(WINDOW_WIDTH.try_into().unwrap(), WINDOW_HEIGHT.try_into().unwrap(), "Minecraft", glfw::WindowMode::Windowed).unwrap();
    window.make_current();
    
    gl::load_renderer();

    let triangle: [Vertex; 4] = [
        Vertex {
            position: [-0.5, -0.5, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 0.0]
        },
        Vertex {
            position: [-0.5, 0.5, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 1.0]
        },
        Vertex {
            position: [0.5, 0.5, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [1.0, 1.0]
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [1.0, 0.0]
        }
    ];

    let indices = [
        0u32, 2, 1,
        0, 3, 2
    ];

    load_gl();

    let test = Drawable::new(&Vec::from(triangle), &Vec::from(indices), "textures/monke.png");
    let test2 = Drawable::new(&vec![
        Vertex {
            position: [-2.0, -0.5, -1.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 0.0]
        },
        Vertex {
            position: [-2.0, 0.5, -1.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 1.0]
        },
        Vertex {
            position: [-1.0, 0.5, -1.0],
            normal: [0.0, 0.0, 1.0],
            uv: [1.0, 1.0]
        },
        Vertex {
            position: [-1.0, -0.5, -1.0],
            normal: [0.0, 0.0, 1.0],
            uv: [1.0, 0.0]
        }
    ], &Vec::from(indices), "textures/monke.png");

    let mut position = [0.0, 0.0, 2.0];
    let mut direction = [0.0, 0.0, -1.0];
    create_projection();
    create_view_matrix(&position, &direction);

    while !window.should_close() {
        glfw.poll_events();
        clear_screen();

        if window.get_key(glfw::Key::Escape) == Action::Press {
            window.set_should_close(true);
        }
        
        inputs(&mut position, &mut direction, &mut window);

        test.draw();
        test2.draw();

        window.swap_buffers();
    }
}

static mut s_first_click: bool = true;

fn inputs(pos: &mut [f32; 3], dir: &mut [f32; 3], window: &mut Window) {
    let first_click: &mut bool = unsafe {
        &mut s_first_click
    };
    
    let mut position = vec3(pos[0], pos[1], pos[2]);
    let mut direction = vec3(dir[0], dir[1], dir[2]);
    let mut up = vec3(0.0, 1.0, 0.0f32);
    let speed: f32 = 0.006;
    let sensitivity: f64 = 100.0;

    use glfw::Action::*;
    use glfw::Key::*;
    if window.get_key(W) == Press {
        position += speed * direction;
    }

    if window.get_key(S) == Press {
        position -= speed * direction;
    }

    if window.get_key(A) == Press {
        position -= speed * glm::cross(&direction, &up);
    }

    if window.get_key(D) == Press {
        position += speed * glm::cross(&direction, &up);
    }

    if window.get_key(E) == Press {
        position += speed * up;
    }

    if window.get_key(Q) == Press {
        position -= speed * up;
    }

    if window.get_mouse_button(glfw::MouseButton::Button2) == Press {
        window.set_cursor_mode(glfw::CursorMode::Hidden);
        if *first_click {
            window.set_cursor_pos((WINDOW_WIDTH / 2) as f64, (WINDOW_HEIGHT / 2) as f64);
           *first_click = false;
        }

        let mut mouseX = 0.0;
        let mut mouseY = 0.0;
        (mouseX, mouseY) = window.get_cursor_pos();

        let rotX = sensitivity * ((mouseY - (WINDOW_HEIGHT / 2) as f64) / WINDOW_HEIGHT as f64);
        let rotY = sensitivity * ((mouseX - (WINDOW_WIDTH / 2) as f64) / WINDOW_WIDTH as f64);

        let newDirection = glm::rotate_vec3(&direction, (-rotX).to_radians() as f32, &glm::cross(&direction, &up));
    
        direction = newDirection;

        direction = glm::rotate_vec3(&direction, (-rotY).to_radians() as f32, &up);
        window.set_cursor_pos((WINDOW_WIDTH / 2) as f64, (WINDOW_HEIGHT / 2) as f64);
    }

    else if window.get_mouse_button(glfw::MouseButton::Button2) == Release {
        window.set_cursor_mode(glfw::CursorMode::Normal);
        *first_click = true;
    }

    *pos = position.into();
    *dir = direction.into();
    create_view_matrix(pos, dir);
}
