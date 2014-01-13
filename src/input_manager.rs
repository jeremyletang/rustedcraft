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

use std::num;

use glfw;
use math::Vec2;
use timer::Timer;

#[deriving(Clone, ToStr)]
pub struct InputDatas {
    keys:           ~[(glfw::Action, glfw::Key)],
    mouse_position: Vec2<f32>
}

pub struct InputManager {
    priv key_port:      Port<(glfw::Action, glfw::Key)>,
    priv timer:         Timer
}


impl InputManager {
    pub fn new(k_port: Port<(glfw::Action, glfw::Key)>) -> InputManager {
        InputManager {
            key_port:       k_port,
            timer:          Timer::new()
        }
    }

    pub fn check_input(&mut self, 
        window: &glfw::Window,
        keys: &mut ~[(glfw::Action, glfw::Key)]) -> () {

        match window.get_key(glfw::KeyW) { glfw::Press => keys.push((glfw::Press, glfw::KeyW)), _ => {} }
        match window.get_key(glfw::KeyD) { glfw::Press => keys.push((glfw::Press, glfw::KeyD)), _ => {} }
        match window.get_key(glfw::KeyS) { glfw::Press => keys.push((glfw::Press, glfw::KeyS)), _ => {} }
        match window.get_key(glfw::KeyA) { glfw::Press => keys.push((glfw::Press, glfw::KeyA)), _ => {} }

    }

    pub fn update(&mut self, window: &glfw::Window) -> InputDatas {
        let mut inputs: ~[(glfw::Action, glfw::Key)] = ~[];

        loop {
            match self.key_port.try_recv() {
                Some((a, k))    => { inputs.push((a, k)) },
                None            => break
            }
        }

        let mouse_pos = match window.get_cursor_pos() {
            (x, y)  => { // println!("MOUSE_X: {:f} / MOUSE_Y: {:f}", x, y);
                if (x == 0f64 && y == 0f64) {
                    Vec2::new(1024f32 / 2f32, 768f32 /2f32)
                } else {
                    Vec2::new(num::cast::<f64,f32>(x).unwrap(), num::cast::<f64, f32>(y).unwrap())
                }
            }
        };
        // if self.timer.get_elapsed_time() > 1f64 {
            match window.get_size() {
                (x, y)  => {
                    window.is_focused();
                    window.set_cursor_pos(num::cast::<i32, f64>(x).unwrap() / 2f64,
                        num::cast::<i32, f64>(y).unwrap() / 2f64);
                    window.is_focused();
                }
            };
            // self.timer.reset()
        // }
        self.check_input(window, &mut inputs);
        InputDatas {
            keys:           inputs,
            mouse_position: mouse_pos
        }        
    }   
}

