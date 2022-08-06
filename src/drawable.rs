use std::{mem::size_of_val, mem::size_of, ffi::CString};

use crate::{Vertex, gl::{create_vao, bind_vao, create_vbo, bind_vbo, vbo_data, create_ebo, bind_ebo, ebo_data, link_attrib, create_texture, create_shader, use_shader, bind_texture, set_texture_uniform, draw}};

#[derive(Debug, Clone)]
pub struct Drawable {
    vertices_count: u32,
    program: u32,
    texture: u32,
    vao: u32,
    vbo: u32,
}

impl Drawable {
    pub fn new(vertices: &Vec<Vertex>, image_path: &str) -> Self {
        let program = create_shader();
        use_shader(program);
        
        let vao: u32 = create_vao();
        bind_vao(vao);

        let vbo: u32 = create_vbo();
        bind_vbo(vbo);
        vbo_data(size_of_val(vertices.as_slice()) as i32, vertices.as_ptr().cast());

        //aPos
        link_attrib(0, 3, size_of::<Vertex>() as i32, 0);
        //aNormal
        link_attrib(1, 3, size_of::<Vertex>() as i32, (size_of::<f32>() * 3) as u32);
        //aUV
        link_attrib(2, 2, size_of::<Vertex>() as i32, (size_of::<f32>() * 6) as u32);

        bind_vao(0);
        bind_vbo(0);
        bind_ebo(0);

        let texture: u32 = create_texture(CString::new(image_path).unwrap());
        bind_texture(texture);
        set_texture_uniform(program);
        
        Drawable {
            vertices_count: vertices.len() as u32,
            program,
            texture,
            vao,
            vbo
        }
    }

    pub fn change_vertices(&mut self, vertices: &Vec<Vertex>) {
        self.vertices_count = vertices.len() as u32;

        bind_vao(self.vao);
        bind_vbo(self.vbo);
        vbo_data(size_of_val(vertices.as_slice()) as i32, vertices.as_ptr().cast());

        bind_vao(self.vao);
        bind_vbo(self.vbo);
    }

    pub fn draw(&self) {
        draw(self.program, self.texture, self.vao, self.vertices_count);
    }
}
