// The MIT License (MIT)
//
// Copyright (c) 2014 Jeremy Letang
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::io::File;
use std::path::Path;
use std::{ptr, str, vec};

use gl;
use gl::types::{GLuint, GLint, GLchar};

pub fn load_from_file(vertex_shader_path: Path, fragment_shader_path: Path) -> Option<GLuint> {
    let vs_str = File::open(&vertex_shader_path).read_to_str();
    let fs_str = File::open(&fragment_shader_path).read_to_str();

    match compile(vs_str, fs_str) {
        Some((vs_id, fs_id))    => link_program(vs_id, fs_id),
        None                    => return None
    }
}

pub fn load_from_string(vertex_shader: &str, fragment_shader: &str) -> Option<GLuint> {
    match compile(vertex_shader, fragment_shader) {
        Some((vs_id, fs_id))    => link_program(vs_id, fs_id),
        None                    => return None
    }
}

fn compile(vs_str: &str, fs_str: &str) -> Option<(GLuint, GLuint)> {
    let vs_id: GLuint = gl::CreateShader(gl::VERTEX_SHADER);
    let fs_id: GLuint = gl::CreateShader(gl::FRAGMENT_SHADER);

    vs_str.with_c_str(|c_str| {
            unsafe { gl::ShaderSource(vs_id, 1, &c_str, ptr::null()) };
    });
    gl::CompileShader(vs_id);
    check_error(vs_id);

    fs_str.with_c_str(|c_str| {
            unsafe { gl::ShaderSource(fs_id, 1, &c_str, ptr::null()) };
    });
    gl::CompileShader(fs_id);
    check_error(fs_id);

    Some((vs_id, fs_id))
}

fn check_error(id: GLuint) -> () {
    let mut err = gl::FALSE as GLint;

    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut err);
        if err != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec::from_elem(len as uint - 1, 0u8);
            gl::GetShaderInfoLog(id, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
            fail!(str::raw::from_utf8(buf).to_owned());
        }
    }
}

fn link_program(vs_id: GLuint, fs_id: GLuint) -> Option<GLuint> {
    // link program
    let prog_id: GLuint = gl::CreateProgram();
    gl::AttachShader(prog_id, vs_id);
    gl::AttachShader(prog_id, fs_id);
    gl::LinkProgram(prog_id);
    // check_error(prog_id);

    gl::DeleteShader(vs_id);
    gl::DeleteShader(fs_id);
    Some(prog_id)
}