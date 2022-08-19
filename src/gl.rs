#![allow(non_upper_case_globals)]

use std::{ffi::{CString, c_void}, os::raw::c_char};
use libloading::{Library, Symbol};

pub const GL_ARRAY_BUFFER: u32 = 0x8892;
pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;

pub fn load_dll() {
    unsafe {
        rlib = Some(Library::new("renderer.dll").unwrap());
        let lib_ref = rlib.as_ref().unwrap();
        rload_gl = Some(lib_ref.get(b"load_gl").unwrap());
        rclear_screen = Some(lib_ref.get(b"clear_screen").unwrap());
        rcreate_shader = Some(lib_ref.get(b"create_shader").unwrap());
        ruse_shader = Some(lib_ref.get(b"use_shader").unwrap());
        rcreate_vao = Some(lib_ref.get(b"create_vao").unwrap());
        rbind_vao = Some(lib_ref.get(b"bind_vao").unwrap());
        rlink_attrib = Some(lib_ref.get(b"link_attrib").unwrap());
        rcreate_buffer = Some(lib_ref.get(b"create_buffer").unwrap());
        rbind_buffer = Some(lib_ref.get(b"bind_buffer").unwrap());
        rbuffer_data = Some(lib_ref.get(b"buffer_data").unwrap());
        rdraw_array = Some(lib_ref.get(b"draw_array").unwrap());
        rdraw_element = Some(lib_ref.get(b"draw_element").unwrap());
        rcreate_texture = Some(lib_ref.get(b"create_texture").unwrap());
        rbind_texture = Some(lib_ref.get(b"bind_texture").unwrap());
        rset_texture_uniform = Some(lib_ref.get(b"set_texture_uniform").unwrap());
        rset_matrix_uniform = Some(lib_ref.get(b"set_matrix_uniform").unwrap());
    }
}

//r stands for raw
static mut rlib: Option<Library> = None;
static mut rload_gl: Option<Symbol<fn()>> = None;
static mut rclear_screen: Option<Symbol<fn()>> = None;
static mut rcreate_shader: Option<Symbol<fn(*const c_char, *const c_char) -> u32>> = None;
static mut ruse_shader: Option<Symbol<fn(u32)>> = None;
static mut rcreate_vao: Option<Symbol<fn() -> u32>> = None;
static mut rbind_vao: Option<Symbol<fn(u32)>> = None;
static mut rlink_attrib: Option<Symbol<fn(i32, i32, i32, u32)>> = None;
static mut rcreate_buffer: Option<Symbol<fn(u32) -> u32>> = None;
static mut rbind_buffer: Option<Symbol<fn(u32, u32)>> = None;
static mut rbuffer_data: Option<Symbol<fn(u32, i32, *const c_void)>> = None;
static mut rdraw_array: Option<Symbol<fn(u32)>> = None;
static mut rdraw_element: Option<Symbol<fn(u32)>> = None;
static mut rcreate_texture: Option<Symbol<fn(image_path: *const c_char) -> u32>> = None;
static mut rbind_texture: Option<Symbol<fn(texture: u32) -> ()>> = None;
static mut rset_texture_uniform: Option<Symbol<fn(shader: u32) -> ()>> = None;
static mut rset_matrix_uniform: Option<Symbol<fn(u32, *const c_char, [f32; 16])>> = None;

pub fn load_gl() {
    unsafe {
        rload_gl.as_ref().unwrap()()
    }
}

pub fn clear_screen() {
    unsafe {
        rclear_screen.as_ref().unwrap()()
    }
}

pub fn create_shader(vertex_path: CString, fragment_path: CString) -> u32 {
    unsafe {
        rcreate_shader.as_ref().unwrap()(vertex_path.as_ptr(), fragment_path.as_ptr())
    }
}

pub fn use_shader(shader: u32) {
    unsafe {
        ruse_shader.as_ref().unwrap()(shader)
    }
}

pub fn create_vao() -> u32 {
    unsafe {
        rcreate_vao.as_ref().unwrap()()
    }
}

pub fn bind_vao(vao: u32) {
    unsafe {
        rbind_vao.as_ref().unwrap()(vao)
    }
}

pub fn link_attrib(layout: i32, components: i32, stride: i32, byte_offset: u32) {
    unsafe {
        rlink_attrib.as_ref().unwrap()(layout, components, stride, byte_offset)
    }
}

pub fn create_buffer(buffer_type: u32) -> u32 {
    unsafe {
        rcreate_buffer.as_ref().unwrap()(buffer_type)
    }
}

pub fn bind_buffer(buffer: u32, buffer_type: u32) {
    unsafe {
        rbind_buffer.as_ref().unwrap()(buffer, buffer_type)
    }
}

pub fn buffer_data(buffer_type: u32, size: i32, data: *const c_void) {
    unsafe {
        rbuffer_data.as_ref().unwrap()(buffer_type, size, data)
    }
}

pub fn draw_array(count: u32) {
    unsafe {
        rdraw_array.as_ref().unwrap()(count)
    }
}

pub fn draw_element(count: u32) {
    unsafe {
        rdraw_element.as_ref().unwrap()(count)
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

pub fn set_matrix_uniform(shader: u32, name: CString, matrix: [f32; 16]) {
    unsafe {
        rset_matrix_uniform.as_ref().unwrap()(shader, name.as_ptr(), matrix)
    }
}
