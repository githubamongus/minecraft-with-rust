#![allow(non_upper_case_globals)]

mod gl;
mod drawable;
mod minecraft;

use drawable::Drawable;
use gl::*;
use glfw::{FAIL_ON_ERRORS, Context, Action, Window};
use glm::*;
use minecraft::Chunk;

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
    load_gl();
    
    let mut chunk = Chunk::new();
    chunk.check_faces();
    
    let mut position = [0.0, 0.0, 2.0];
    let mut direction = [0.0, 0.0, -1.0];
    create_projection();
    create_view_matrix(&position, &direction);

    let test = Drawable::new(&vec![
            Vertex {
                position: [-0.2, -0.2, 0.0],
                normal: [0.0, 0.0, 0.0],
                uv: [0.0, 0.0]
            },
            Vertex {
                position: [0.2, -0.2, 0.0],
                normal: [0.0, 0.0, 0.0],
                uv: [1.0, 0.0]
            },
            Vertex {
                position: [0.0, 0.2, 0.0],
                normal: [0.0, 0.0, 0.0],
                uv: [0.5, 1.0]
            }
        ],
        "textures/speed retarded.png",
        false
    );
    
    let mut prev_time = std::time::Instant::now();
    let mut count = 0;
    while !window.should_close() {
        let crnt_time = std::time::Instant::now();
        count += 1;
        if crnt_time - prev_time >= std::time::Duration::from_secs(1) {
            println!("{} - fps", count);
            count = 0;
            prev_time = crnt_time;
        }

        glfw.poll_events();
        clear_screen();
        
        if window.get_key(glfw::Key::Escape) == Action::Press {
            window.set_should_close(true);
        }
        
        //println!("pos {:?}", position);
        //println!("dir {:?}", direction);
        chunk.draw();
        test.draw();
        inputs(&mut position, &mut direction, &mut window);
        window.swap_buffers();
    }
}

fn inputs(pos: &mut [f32; 3], dir: &mut [f32; 3], window: &mut Window) {
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

    window.set_cursor_mode(glfw::CursorMode::Hidden);

    let mut mouseX = 0.0;
    let mut mouseY = 0.0;
    (mouseX, mouseY) = window.get_cursor_pos();

    let rotX = sensitivity * ((mouseY - (WINDOW_HEIGHT / 2) as f64) / WINDOW_HEIGHT as f64);
    let rotY = sensitivity * ((mouseX - (WINDOW_WIDTH / 2) as f64) / WINDOW_WIDTH as f64);
    let newDirection = glm::rotate_vec3(&direction, (-rotX).to_radians() as f32, &glm::cross(&direction, &up));

    direction = newDirection;
    direction = glm::rotate_vec3(&direction, (-rotY).to_radians() as f32, &up);

    window.set_cursor_pos((WINDOW_WIDTH / 2) as f64, (WINDOW_HEIGHT / 2) as f64);

    *pos = position.into();
    *dir = direction.into();
    create_view_matrix(pos, dir);
}
