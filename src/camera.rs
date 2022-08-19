use std::ffi::CString;

use glfw::Window;
use glm::{Vec3, Mat4, vec3};

use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, gl::{use_shader, set_matrix_uniform}};

pub struct Camera {
    position: Vec3,
    direction: Vec3,
    projection: Mat4,
    view: Mat4
}

impl Camera {
    pub fn new(position: Vec3) -> Self {
        Camera {
            position,
            direction: vec3(0.0, 0.0, -1.0f32),
            projection: glm::perspective(WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32, 45.0f32.to_radians(), 0.1, 100.0),
            view: glm::look_at(&position, &(position + vec3(0.0, 0.0, -1.0)), &vec3(0.0, 1.0, 0.0))
        }
    }

    pub fn inputs(&mut self, window: &mut Window, delta_time: f32) {
        if window.is_focused() {
            let up = vec3(0.0, 1.0, 0.0f32);
            let speed: f32 = 4.0;
            let sensitivity: f64 = 120.0;

            use glfw::Action::*;
            use glfw::Key::*;
            if window.get_key(W) == Press {
                self.position += speed * delta_time * self.direction;
            }

            if window.get_key(S) == Press {
                self.position -= speed * delta_time * self.direction;
            }

            if window.get_key(A) == Press {
                self.position -= speed * delta_time * glm::normalize(&glm::cross(&self.direction, &up));
            }

            if window.get_key(D) == Press {
                self.position += speed * delta_time * glm::normalize(&glm::cross(&self.direction, &up));
            }

            if window.get_key(E) == Press {
                self.position += speed * delta_time * up;
            }

            if window.get_key(Q) == Press {
                self.position -= speed * delta_time * up;
            }

            window.set_cursor_mode(glfw::CursorMode::Hidden);

            let mut mouseX = 0.0;
            let mut mouseY = 0.0;
            (mouseX, mouseY) = window.get_cursor_pos();

            let rotX = sensitivity * ((mouseY - (WINDOW_HEIGHT / 2) as f64) / WINDOW_HEIGHT as f64);
            let rotY = sensitivity * ((mouseX - (WINDOW_WIDTH / 2) as f64) / WINDOW_WIDTH as f64);
            let newDirection = glm::rotate_vec3(&self.direction, (-rotX).to_radians() as f32, &glm::cross(&self.direction, &up));

            if newDirection.y <= 0.975 && newDirection.y >= -0.975 {
                self.direction = newDirection;
            }
            self.direction = glm::rotate_vec3(&self.direction, (-rotY).to_radians() as f32, &up);

            window.set_cursor_pos((WINDOW_WIDTH / 2) as f64, (WINDOW_HEIGHT / 2) as f64);
        }

        self.view = glm::look_at(&self.position, &(self.position + self.direction), &vec3(0.0, 1.0, 0.0));
    }

    pub fn set_uniforms(&self, shader: u32) {
        use_shader(shader);
        set_matrix_uniform(shader, CString::new("projection").unwrap(), glm::value_ptr(&self.projection).try_into().unwrap());
        set_matrix_uniform(shader, CString::new("view").unwrap(), glm::value_ptr(&self.view).try_into().unwrap());
    }
}
