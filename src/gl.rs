#![allow(non_upper_case_globals)]
use std::{ffi::{c_void, CString}, os::raw::c_char, ops::Deref};

use libloading::{Library, Symbol};

pub fn load_renderer() {
    unsafe {
        lib = Some(Library::new("renderer.dll").unwrap());
        
        rload_gl = Some(lib.as_ref().unwrap().get(b"load_gl").unwrap());
        rclear_screen = Some(lib.as_ref().unwrap().get(b"clear_screen").unwrap());
        rcreate_shader = Some(lib.as_ref().unwrap().get(b"create_shader").unwrap());
        ruse_shader = Some(lib.as_ref().unwrap().get(b"use_shader").unwrap());
        rcreate_vao = Some(lib.as_ref().unwrap().get(b"create_vao").unwrap());
        rbind_vao = Some(lib.as_ref().unwrap().get(b"bind_vao").unwrap());
        rlink_attrib = Some(lib.as_ref().unwrap().get(b"link_attrib").unwrap());
        rcreate_vbo = Some(lib.as_ref().unwrap().get(b"create_vbo").unwrap());
        rbind_vbo = Some(lib.as_ref().unwrap().get(b"bind_vbo").unwrap());
        rvbo_data = Some(lib.as_ref().unwrap().get(b"vbo_data").unwrap());
        rcreate_ebo = Some(lib.as_ref().unwrap().get(b"create_ebo").unwrap());
        rbind_ebo = Some(lib.as_ref().unwrap().get(b"bind_ebo").unwrap());
        rebo_data = Some(lib.as_ref().unwrap().get(b"ebo_data").unwrap());
        rcreate_texture = Some(lib.as_ref().unwrap().get(b"create_texture").unwrap());
        rbind_texture = Some(lib.as_ref().unwrap().get(b"bind_texture").unwrap());
        rset_texture_uniform = Some(lib.as_ref().unwrap().get(b"set_texture_uniform").unwrap());
        rdraw = Some(lib.as_ref().unwrap().get(b"draw").unwrap());
        rcreate_projection = Some(lib.as_ref().unwrap().get(b"create_projection").unwrap());
        rcreate_view_matrix = Some(lib.as_ref().unwrap().get(b"create_view_matrix").unwrap());
    }
}

static mut lib: Option<Library> = None;

pub static mut rload_gl: Option<Symbol<fn() -> ()>> = None;
pub static mut rclear_screen: Option<Symbol<fn() -> ()>> = None;
pub static mut rcreate_shader: Option<Symbol<fn() -> u32>> = None;
pub static mut ruse_shader: Option<Symbol<fn(shader: u32) -> ()>> = None;
pub static mut rcreate_vao: Option<Symbol<fn() -> u32>> = None;
pub static mut rbind_vao: Option<Symbol<fn(vao: u32) -> ()>> = None;
pub static mut rlink_attrib: Option<Symbol<fn(layout: i32, components: i32, stride: i32, byte_offset: u32) -> ()>> = None;
pub static mut rcreate_vbo: Option<Symbol<fn() -> u32>> = None;
pub static mut rbind_vbo: Option<Symbol<fn(vbo: u32) -> ()>> = None;
pub static mut rvbo_data: Option<Symbol<fn(size: i32, data: *const c_void)>> = None;
pub static mut rcreate_ebo: Option<Symbol<fn() -> u32>> = None;
pub static mut rbind_ebo: Option<Symbol<fn(ebo: u32) -> ()>> = None;
pub static mut rebo_data: Option<Symbol<fn(size: i32, data: *const c_void)>> = None;
pub static mut rcreate_texture: Option<Symbol<fn(image_path: *const c_char) -> u32>> = None;
pub static mut rbind_texture: Option<Symbol<fn(texture: u32) -> ()>> = None;
pub static mut rset_texture_uniform: Option<Symbol<fn(shader: u32) -> ()>> = None;
pub static mut rdraw: Option<Symbol<fn(shader: u32, texture: u32, vao: u32, vertices_count: u32) -> ()>> = None;
pub static mut rcreate_projection: Option<Symbol<fn() -> ()>> = None;
pub static mut rcreate_view_matrix: Option<Symbol<fn(position: [f32; 3], direction: [f32; 3]) -> ()>> = None;

pub fn load_gl() {
    unsafe {
        rload_gl.as_ref().unwrap()();
    }
}

pub fn clear_screen() {
    unsafe {
        rclear_screen.as_ref().unwrap()();
    }
}

pub fn create_shader() -> u32 {
    unsafe {
        rcreate_shader.as_ref().unwrap()()
    }
}

pub fn use_shader(shader: u32) {
    unsafe {
        ruse_shader.as_ref().unwrap()(shader);
    }
}

pub fn create_vao() -> u32 {
    unsafe {
        rcreate_vao.as_ref().unwrap()()
    }
}

pub fn bind_vao(vao: u32) {
    unsafe {
        rbind_vao.as_ref().unwrap()(vao);
    }
}

pub fn link_attrib(layout: i32, components: i32, stride: i32, byte_offset: u32) {
    unsafe {
        rlink_attrib.as_ref().unwrap()(layout, components, stride, byte_offset);
    }
}

pub fn create_vbo() -> u32 {
    unsafe {
        rcreate_vbo.as_ref().unwrap()()
    }
}

pub fn bind_vbo(vbo: u32) {
    unsafe {
        rbind_vbo.as_ref().unwrap()(vbo);
    }
}

pub fn vbo_data(size: i32, data: *const c_void) {
    unsafe {
        rvbo_data.as_ref().unwrap()(size, data);
    }
}

pub fn create_ebo() -> u32 {
    unsafe {
        rcreate_ebo.as_ref().unwrap()()
    }
}

pub fn bind_ebo(ebo: u32) {
    unsafe {
        rbind_ebo.as_ref().unwrap()(ebo);
    }
}

pub fn ebo_data(size: i32, data: *const c_void) {
    unsafe {
        rebo_data.as_ref().unwrap()(size, data);
    }
}

pub fn create_texture(image_path: CString) -> u32 {
    unsafe {
        rcreate_texture.as_ref().unwrap()(image_path.as_ptr())
    }
}

pub fn bind_texture(texture: u32) {
    unsafe {
        rbind_texture.as_ref().unwrap()(texture);
    }
}

pub fn set_texture_uniform(shader: u32) {
    unsafe {
        rset_texture_uniform.as_ref().unwrap()(shader);
    }
}

pub fn create_projection() {
    unsafe {
        rcreate_projection.as_ref().unwrap()();
    }
}

pub fn create_view_matrix(position: &[f32; 3], direction: &[f32; 3]) {
    unsafe {
        rcreate_view_matrix.as_ref().unwrap()(position.clone(), direction.clone());
    }
}

pub fn draw(shader: u32, texture: u32, vao: u32, vertices_count: u32) {
    unsafe {
        rdraw.as_ref().unwrap()(shader, texture, vao, vertices_count);
    }
}
