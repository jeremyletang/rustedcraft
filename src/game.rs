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

use gl;
use gl::types::GLuint;
use glfw;

use glfw_utils;
use texture_loader;
use texture_loader::TextureLoader;
use world::World;
use font::Font;
use perf_metrics::{PerfMetrics, Fps};
use input_manager::{InputManager, InputDatas};

pub struct Game {
    priv window:            glfw::Window,
    priv texture_loader:    Rc<RefCell<TextureLoader>>,
    priv world:             World,
    priv game_font:         Rc<RefCell<Font>>,
    priv fps:               PerfMetrics<Fps>,
    priv vertex_array:      GLuint,
    priv input_manager:     InputManager

}

impl Game {
    pub fn new() -> Game {
        let window =            glfw_utils::init();
        let vertex_array =      Game::init_gl();
        let input_manager =     glfw_utils::init_callbacks(&window);
        let tex_loader =        texture_loader::make();
        let world =             World::new(tex_loader.clone());
        let font =              Rc::from_mut(RefCell::new(Font::new()));
        let pm =                PerfMetrics::new(Fps::new(), font.clone());

        Game {
            window:             window,
            texture_loader:     tex_loader,
            world:              world,
            game_font:          font,
            fps:                pm,
            vertex_array:       vertex_array,
            input_manager:      input_manager
        }
    }

    fn init_gl() -> GLuint {
        gl::load_with(glfw::get_proc_address);
        gl::ClearColor(0., 0., 0.4, 0.);
        let mut vertex_array =  0;
        unsafe { 
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::BindVertexArray(vertex_array);
        }     
        gl::Enable(gl::DEPTH_TEST);
        // Accept fragment if it closer to the camera than the former one
        gl::DepthFunc(gl::LESS);

        vertex_array
    }

    pub fn test_should_close(&mut self, inputs: &InputDatas) -> () {
        for k in inputs.keys.iter() {
            match k {
                &(_, glfw::KeyEscape)    => self.window.set_should_close(true),
                _                       => {}
            }
        }
    }

    pub fn run(&mut self) -> () {
        let mut input_datas: InputDatas;
        while !self.window.should_close() {
            self.fps.frame_begin();
            // Poll events
            glfw::poll_events();
            input_datas = self.input_manager.update(&self.window);
            self.test_should_close(&input_datas);
            
            // Clear the screen to black
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            self.world.draw();

            self.fps.frame_end();
            self.fps.draw();

            // Swap buffers
            self.window.swap_buffers();
        }
    }
}

#[unsafe_destructor]
impl Drop for Game {
    fn drop(&mut self) -> () {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vertex_array);
        }
    }
}
