use std::ffi::CString;
use std::mem::{size_of, size_of_val};

use glm::{Vec3, Vec2, Mat4};

use crate::camera::Camera;
use crate::gl::{create_shader, use_shader, create_vao, bind_vao, create_buffer, bind_buffer, link_attrib, buffer_data, draw_element, draw_array, create_texture, bind_texture, set_texture_uniform, set_matrix_uniform};
use crate::{GL_ARRAY_BUFFER, GL_ELEMENT_ARRAY_BUFFER};

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2
}

#[derive(Debug)]
pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
    texture: u32,
    shader: u32,
    has_indices: bool,
    count: u32,
    model: Mat4
}

impl Mesh {
    pub fn new(image_path: &str) -> Self {
        let shader = create_shader(CString::new("shaders/default.vert").unwrap(), CString::new("shaders/default.frag").unwrap());
        use_shader(shader);

        let vao = create_vao();
        bind_vao(vao);

        let vbo = create_buffer(GL_ARRAY_BUFFER);
        bind_buffer(vbo, GL_ARRAY_BUFFER);
        let ebo = create_buffer(GL_ELEMENT_ARRAY_BUFFER);
        bind_buffer(ebo, GL_ELEMENT_ARRAY_BUFFER);

        link_attrib(0,3, size_of::<Vertex>() as i32, 0);
        link_attrib(1, 3, size_of::<Vertex>() as i32, size_of::<f32>() as u32 * 3);
        link_attrib(2, 2, size_of::<Vertex>() as i32, size_of::<f32>() as u32 * 6);

        let texture = create_texture(CString::new(image_path).unwrap());
        bind_texture(texture);
        set_texture_uniform(shader);

        bind_vao(0);
        bind_buffer(0, GL_ARRAY_BUFFER);
        bind_buffer(0, GL_ELEMENT_ARRAY_BUFFER);
        bind_texture(0);

        Mesh {
            vao,
            vbo,
            ebo,
            texture,
            shader,
            has_indices: false,
            count: 0,
            model: Mat4::new_scaling(1.0)
        }
    }

    pub fn set_vbo_data(&mut self, vertices: &[Vertex]) {
        //make a vector to convert glm::vec3 into primitive types
        let vertices = {
            let mut temp: Vec<([f32; 3], [f32; 3], [f32; 2])> = Vec::new();

            for vertex in vertices {
                temp.push((vertex.position.into(), vertex.normal.into(), vertex.uv.into()));
            }

            temp
        };
        //extract slice out of the vector and drop the vector
        let vertices = vertices.as_slice();
        bind_buffer(self.vbo, GL_ARRAY_BUFFER);
        buffer_data(GL_ARRAY_BUFFER, size_of_val(vertices) as i32, vertices.as_ptr().cast());
        bind_buffer(0, GL_ARRAY_BUFFER);
        self.count = vertices.len() as u32;
    }

    pub fn set_ebo_data(&mut self, indices: &[u32]) {
        bind_buffer(self.ebo, GL_ELEMENT_ARRAY_BUFFER);
        buffer_data(GL_ELEMENT_ARRAY_BUFFER, size_of_val(indices) as i32, indices.as_ptr().cast());
        bind_buffer(0, GL_ELEMENT_ARRAY_BUFFER);
        self.has_indices = true;
        self.count = indices.len() as u32;
    }

    pub fn draw(&self, camera: &Camera) {
        use_shader(self.shader);
        bind_vao(self.vao);
        bind_texture(self.texture);
        camera.set_uniforms(self.shader);
        set_matrix_uniform(self.shader, CString::new("model").unwrap(), glm::value_ptr(&self.model).try_into().unwrap());
        if self.has_indices {
            draw_element(self.count);
        } else {
            draw_array(self.count);
        }
    }
}
