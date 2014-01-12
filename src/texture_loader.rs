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

use std::rc::Rc;
use std::cell::RefCell;
use std::cast;

use gl;
use gl::types::{GLuint, GLenum};
use stb;
use stb::image::ImageU8;

pub fn make() -> Rc<RefCell<TextureLoader>> {
    let mut tex_loader = TextureLoader::new();    
    tex_loader.load(~"./assets/ascii.png", gl::RGBA); 
    tex_loader.load(~"./assets/tex.jpg", gl::RGB);
    tex_loader.load(~"./assets/stonebrick_cracked.png", gl::RGBA);
    Rc::from_mut(RefCell::new(tex_loader))
}

pub struct TextureLoader {
    priv textures: ~[GLuint]
}

impl TextureLoader {
    pub fn new() -> TextureLoader {
        TextureLoader {
            textures: ~[]
        }
    }

    pub fn load(&mut self, 
        texture_path: ~str, 
        color_mod: GLenum) -> () {

        self.textures.push(load_texture(texture_path, color_mod));
    }

    pub fn get(&self, index: u32) -> GLuint {
        self.textures[index]
    }
}

pub fn load_texture(texture_path: ~str, color_mod: GLenum) -> GLuint {
    let raw_tex = match stb::image::load(texture_path.clone()) {
    ImageU8(i)  => i,
    _           => fail!("Cannot load asset: {}", texture_path)
    };
    let mut tex = 0;
    unsafe {
        gl::GenTextures(1, &mut tex);
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexImage2D(gl::TEXTURE_2D, 
            0, 
            color_mod as i32, 
            raw_tex.width as i32, 
            raw_tex.height as i32, 
            0, 
            color_mod, 
            gl::UNSIGNED_BYTE, 
            cast::transmute(raw_tex.data.as_ptr()));
        gl::TexParameteri(gl::TEXTURE_2D, 
            gl::TEXTURE_MAG_FILTER, 
            gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, 
            gl::TEXTURE_MIN_FILTER, 
            gl::NEAREST as i32);
    }
    tex
}

impl Drop for TextureLoader {
    fn drop(&mut self) -> () {
        for t in self.textures.iter() {
             unsafe { gl::DeleteTextures(1, t); }
        }
    }
}