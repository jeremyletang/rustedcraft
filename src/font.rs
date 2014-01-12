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

use gl;
use gl::types::{GLuint, GLint};

use texture_loader;
use shaders;

pub struct Font {
    priv prog_id:       GLuint,
    // texture from stb_image loaded in openGL
    priv tex_id:        GLuint,
    // texture_id in the shader
    priv tex_uniform:   GLint
}

impl Font {
    pub fn new() -> Font {
        let prog = shaders::load_from_file(
            Path::new("./shaders/text_vertex_shader.glsl"), 
            Path::new("./shaders/text_fragment_shader.glsl")
        ).unwrap();

        let tex = texture_loader::load_texture(~"./assets/ascii.png", gl::RGBA);
        let tex_uni = unsafe { "font_texture".with_c_str(|c_str| gl::GetUniformLocation(prog, c_str)) };

        Font {
            prog_id:    prog,
            tex_id:     tex,
            tex_uniform:tex_uni
        }
    }

    pub fn get_progid(&self) -> GLuint {
        self.prog_id
    }

    pub fn get_texid(&self) -> GLuint {
        self.tex_id
    }

    pub fn get_texuniform(&self) -> GLint {
        self.tex_uniform
    }
}

impl Drop for Font {
    fn drop(&mut self) -> () {
        unsafe {
            gl::DeleteTextures(1, &self.tex_id);
            gl::DeleteProgram(self.prog_id);
        }
    }
}