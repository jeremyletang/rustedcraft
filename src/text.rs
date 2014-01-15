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

// TODO : return Vec4<f32>(left, top, width, height)
// TODO text::string_size(string: &str) -> Vec4<f32>;


use std::rc::Rc;
use std::cell::RefCell;
use std::{ptr, mem, cast};
use std::num;

use gl;
use gl::types::{GLuint, GLsizeiptr};

use math::Vec2;
use font::Font;

pub struct Text {
    priv font:          Rc<RefCell<Font>>,
    priv text:          Option<~str>,
    priv vertices:      GLuint,
    priv tex_coords:    GLuint, 
    priv vertices_size: uint,
    priv position:      Vec2<f32>,
    priv size:          uint
}

impl Text {
    pub fn new(font: Rc<RefCell<Font>>, 
        text: Option<~str>, 
        text_position: Vec2<f32>) -> Text {

        let mut v = 0;
        let mut t = 0;

        unsafe {
            gl::GenBuffers(1, &mut v);
            gl::GenBuffers(1, &mut t);
        }
        let mut ret_text = Text {
            font:           font,
            text:           text.clone(),
            vertices:       v,
            tex_coords:     t,
            vertices_size:  0,
            position:       text_position,
            size:           12
        };
        ret_text.set_text(text.unwrap());
        ret_text
    }

    pub fn clear(&mut self) -> () {
        self.text = None;
    }

    pub fn get_text(&self) -> Option<~str> {
        self.text.clone()
    }

    pub fn set_size(&mut self, new_size: uint) -> () {
        self.size = new_size;
    }

    pub fn set_position(&mut self, new_position: &Vec2<f32>) -> () {
        self.position = new_position.clone()
    }

    pub fn draw(&self) -> () {
        if self.text.is_some() {
            gl::UseProgram(self.font.borrow().with(|f| f.get_progid()));

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.font.borrow().with(|f| f.get_texid()));
            gl::Uniform1i(self.font.borrow().with(|f| f.get_texuniform()), 0);

            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertices);
            unsafe { gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, ptr::null()); }

            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tex_coords);
            unsafe { gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, ptr::null()); }

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices_size as i32);

            gl::Disable(gl::BLEND);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
        }
    }

    pub fn set_text(&mut self, text: ~str) -> () {
        let mut vertices:   ~[Vec2<f32>]    = ~[];
        let mut tex_coords: ~[Vec2<f32>]    = ~[];
        let mut iter = 0f32;

        for c in text.bytes() {
            vertices.push(Vec2::<f32>::new(self.position.x + iter * self.size as f32 , self.position.y + self.size as f32));
            vertices.push(Vec2::<f32>::new(self.position.x + iter * self.size as f32 , self.position.y));
            vertices.push(Vec2::<f32>::new(self.position.x + iter * self.size as f32 + self.size as f32, self.position.y + self.size as f32));
        
            vertices.push(Vec2::<f32>::new(self.position.x + iter * self.size as f32 + self.size as f32, self.position.y));
            vertices.push(Vec2::<f32>::new(self.position.x + iter * self.size as f32 + self.size as f32, self.position.y + self.size as f32));
            vertices.push(Vec2::<f32>::new(self.position.x + iter * self.size as f32 , self.position.y));

            let uv_x = (num::cast::<u8, u32>(c).unwrap() % 16) as f32 / 16f32;
            let uv_y = (num::cast::<u8, u32>(c).unwrap() / 16) as f32 / 16f32;

            tex_coords.push(Vec2::<f32>::new(uv_x, 1f32 - uv_y));
            tex_coords.push(Vec2::<f32>::new(uv_x, 1f32 - (uv_y + 1f32 / 16f32)));
            tex_coords.push(Vec2::<f32>::new(uv_x + 1f32 / 16f32, 1f32 - uv_y));

            tex_coords.push(Vec2::<f32>::new(uv_x + 1f32 / 16f32, 1f32 - (uv_y + 1f32 / 16f32)));
            tex_coords.push(Vec2::<f32>::new(uv_x + 1f32 / 16f32, 1f32 - uv_y));
            tex_coords.push(Vec2::<f32>::new(uv_x, 1f32 - (uv_y + 1f32 / 16f32)));

            iter += 1f32;
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertices);
            gl::BufferData(gl::ARRAY_BUFFER, 
                (vertices.len() * mem::size_of::<Vec2<f32>>()) as GLsizeiptr,
                cast::transmute(&vertices[0]), 
                gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tex_coords);
            gl::BufferData(gl::ARRAY_BUFFER, 
                (tex_coords.len() * mem::size_of::<Vec2<f32>>()) as GLsizeiptr,
                cast::transmute(&tex_coords[0]), 
                gl::STATIC_DRAW);
        }
        self.vertices_size = vertices.len();
    }
}

#[unsafe_destructor]
impl Drop for Text {
    fn drop(&mut self) -> () {
        unsafe {
            gl::DeleteBuffers(1, &self.vertices);
            gl::DeleteBuffers(1, &self.tex_coords);
        }
    }
}