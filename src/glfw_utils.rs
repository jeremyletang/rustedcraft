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

use std::libc::c_int;

use glfw;
use input_manager::InputManager;

pub fn init() -> glfw::Window {
    glfw::window_hint::context_version(3, 3);
    glfw::window_hint::opengl_profile(glfw::OpenGlCoreProfile);
    glfw::window_hint::opengl_forward_compat(true);
    glfw::window_hint::samples(2);
    let window = glfw::Window::create(1024, 768, "rustedcraft", glfw::Windowed).unwrap();
    window.make_context_current();
    window.set_cursor_pos(1024f64 / 2f64, 768f64 / 2f64);
    // glfw::set_swap_interval(0);

    window
}

pub fn init_callbacks(window: &glfw::Window) -> InputManager {
    let (key_p, key_c) = Chan::new();

    window.set_key_callback(~KeyCallbackImpl { key_chan: key_c });
    glfw::set_error_callback(~ErrorContext);
    InputManager::new(key_p)
}


struct ErrorContext;
impl glfw::ErrorCallback for ErrorContext {
    fn call(&self, _: glfw::Error, description: ~str) {
        println!("GLFW Error: {:s}", description);
    }
}

struct KeyCallbackImpl { key_chan: Chan<(glfw::Action, glfw::Key)> }
impl glfw::KeyCallback for KeyCallbackImpl {
    fn call(&self, 
        _: &glfw::Window, 
        key: glfw::Key, 
        _: c_int, 
        action: glfw::Action, 
        _: glfw::Modifiers) {

        self.key_chan.send((action, key));
    }
}

