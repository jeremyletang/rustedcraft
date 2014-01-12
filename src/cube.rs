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

use std::{mem, cast, ptr};

use gl;
use gl::types::{GLfloat, GLsizeiptr, GLuint, GLushort, GLint};
use math::Mat4;
use shaders;

pub struct Cube {
    priv vertices:      GLuint,
    priv indices:       GLuint,
    priv tex_coords:    GLuint,
    priv prog:          GLuint,
    priv mvp:           GLint,
    priv tex:           GLint
}

impl Cube {
    pub fn new() -> Cube {
        let prog_id = shaders::load_from_file(
            Path::new("./shaders/cube_vertex_shader.glsl"), 
            Path::new("./shaders/cube_fragment_shader.glsl")
        ).unwrap();
        let mvp_id = unsafe { "mvp".with_c_str(|c_str| gl::GetUniformLocation(prog_id, c_str)) };
        let tex_id = unsafe { "tex".with_c_str(|c_str| gl::GetUniformLocation(prog_id, c_str)) };

        Cube {
            vertices:   make_cube_vertices(),
            indices:    make_cube_indices(),
            tex_coords: make_tex_coords(),
            prog:       prog_id,
            mvp:        mvp_id,
            tex:        tex_id
        }
    }

    pub fn draw_cube(&self, tex_id: GLuint, mvp: &Mat4<f32>) -> () {
        gl::UseProgram(self.prog);
        
        unsafe { gl::UniformMatrix4fv(self.mvp, 1, gl::FALSE, &mvp.a1); }

        // Bind our texture in Texture Unit 0
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, tex_id);
        // Set our "myTextureSampler" sampler to user Texture Unit 0
        gl::Uniform1i(self.tex, 0);

        // cube vertices
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vertices);
        unsafe { gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null()); }
        
        // texture coords
        gl::EnableVertexAttribArray(1);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.tex_coords);
        unsafe { gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, ptr::null()); }    

        // Draw elements
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.indices);
            let mut size = 0;  
            gl::GetBufferParameteriv(gl::ELEMENT_ARRAY_BUFFER, gl::BUFFER_SIZE, &mut size);
            gl::DrawElements(gl::TRIANGLES, size / mem::size_of::<GLushort>() as i32, gl::UNSIGNED_SHORT, ptr::null());
        }
        gl::DisableVertexAttribArray(0);
        gl::DisableVertexAttribArray(1);
    }
}

fn make_cube_vertices() -> GLuint {
    let mut cube_vertices = 0;
    let cube = [
    -1f32, -1f32,  1f32,
     1f32, -1f32,  1f32,
     1f32,  1f32,  1f32,
    -1f32,  1f32,  1f32,
    // top
    -1f32,  1f32,  1f32,
     1f32,  1f32,  1f32,
     1f32,  1f32, -1f32,
    -1f32,  1f32, -1f32,
    // back
     1f32, -1f32, -1f32,
    -1f32, -1f32, -1f32,
    -1f32,  1f32, -1f32,
     1f32,  1f32, -1f32,
    // bottom
    -1f32, -1f32, -1f32,
     1f32, -1f32, -1f32,
     1f32, -1f32,  1f32,
    -1f32, -1f32,  1f32,
    // left
    -1f32, -1f32, -1f32,
    -1f32, -1f32,  1f32,
    -1f32,  1f32,  1f32,
    -1f32,  1f32, -1f32,
    // right
     1f32, -1f32,  1f32,
     1f32, -1f32, -1f32,
     1f32,  1f32, -1f32,
     1f32,  1f32,  1f32];

     // cube_vertices
    unsafe {
        gl::GenBuffers(1, &mut cube_vertices);
        gl::BindBuffer(gl::ARRAY_BUFFER, cube_vertices);
        gl::BufferData(gl::ARRAY_BUFFER, 
            (cube.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, 
            cast::transmute(&cube[0]), 
            gl::STATIC_DRAW);
    }
    cube_vertices
 }


fn make_cube_indices() -> GLuint {
    let mut cube_indices = 0;
    let indices: [GLushort, ..36] = [
    // front
    0,  1,  2,
    2,  3,  0,
    // top
    4,  5,  6,
    6,  7,  4,
    // back
    8,  9, 10,
    10, 11,  8,
    // bottom
    12, 13, 14,
    14, 15, 12,
    // left
    16, 17, 18,
    18, 19, 16,
    // right
    20, 21, 22,
    22, 23, 20];

    unsafe {
        gl::GenBuffers(1, &mut cube_indices);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, cube_indices);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 
            (indices.len() * mem::size_of::<GLushort>()) as GLsizeiptr, 
            cast::transmute(&indices[0]), 
            gl::STATIC_DRAW);
    }
    cube_indices
}

fn make_tex_coords() -> GLuint {
    // front
    let mut tex_coords = 0;
    let coords = [
    0f32, 0f32,
    1f32, 0f32,
    1f32, 1f32,
    0f32, 1f32,
    0f32, 0f32,
    1f32, 0f32,
    1f32, 1f32,
    0f32, 1f32,
    0f32, 0f32,
    1f32, 0f32,
    1f32, 1f32,
    0f32, 1f32,
    0f32, 0f32,
    1f32, 0f32,
    1f32, 1f32,
    0f32, 1f32,
    0f32, 0f32,
    1f32, 0f32,
    1f32, 1f32,
    0f32, 1f32,
    0f32, 0f32,
    1f32, 0f32,
    1f32, 1f32,
    0f32, 1f32,
    0f32, 0f32,
    1f32, 0f32,
    1f32, 1f32,
    0f32, 1f32,
    0f32, 0f32,
    1f32, 0f32,
    1f32, 1f32,
    0f32, 1f32,
    0f32, 0f32,
    1f32, 0f32,
    1f32, 1f32,
    0f32, 1f32];

    unsafe {
        gl::GenBuffers(1, &mut tex_coords);
        gl::BindBuffer(gl::ARRAY_BUFFER, tex_coords);
        gl::BufferData(gl::ARRAY_BUFFER, 
            (coords.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            cast::transmute(&coords[0]), 
            gl::STATIC_DRAW);
    }
    tex_coords
}

impl Drop for Cube {
    fn drop(&mut self) -> () {
        unsafe {
            gl::DeleteBuffers(1, &self.vertices);
            gl::DeleteBuffers(1, &self.tex_coords);
            gl::DeleteBuffers(1, &self.indices);
            gl::DeleteProgram(self.prog);
        }
    }
}
