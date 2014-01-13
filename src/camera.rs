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

use std::num::{cos, sin};

use glfw;

use input_manager::InputDatas;
use math::{Mat4, Vec3, Vec2};
use timer::Timer;

pub static SPEED: f32               = 10f32;
pub static MOUSE_SPEED: f32         = 0.07f32;
pub static FOVY: f32                = 45f32;
pub static ASPECT: f32              = 4f32 / 3f32;
pub static Z_NEAR: f32              = 0.1f32;
pub static Z_FAR: f32               = 100f32;
pub static CONST_RIGHT_ANGLE: f32   = 3.14f32 / 2f32;

pub struct Camera {
    priv mat_projection:    Mat4<f32>,
    priv mat_view:          Mat4<f32>,
    priv position:          Vec3<f32>,
    priv h_angle:           f32,
    priv v_angle:           f32,
    priv timer:             Timer,
    priv window_size:       Vec2<f32>
}

impl Camera {
    pub fn new(window_size: Vec2<f32>) -> Camera {
        Camera {
            mat_projection:     Mat4::perspective(FOVY, ASPECT, Z_NEAR, Z_FAR),
            mat_view:           Mat4::look_at(&Vec3::new(0f32, 0f32, 5f32), &Vec3::new(0f32, 0f32, 0f32), &Vec3::new(0f32, 1f32, 0f32)),
            position:           Vec3::new(0f32, 0f32, 5f32),
            h_angle:            3.14f32,
            v_angle:            0f32,
            timer:              Timer::new(),
            window_size:        window_size
        }
    }

    pub fn move(&mut self, 
        input_datas: &InputDatas,
        dir: &Vec3<f32>,
        right: &Vec3<f32>,
        delta_time: f32) -> () {

        for i in input_datas.keys.iter() {
            match i {
                &(glfw::Press, glfw::KeyW)     => self.position = self.position.add_vec(&dir.scalar_product(delta_time).scalar_product(SPEED)),
                &(glfw::Press, glfw::KeyS)   => self.position = self.position.sub_vec(&dir.scalar_product(delta_time).scalar_product(SPEED)),
                &(glfw::Press, glfw::KeyD)  => self.position = self.position.add_vec(&right.scalar_product(delta_time).scalar_product(SPEED)),
                &(glfw::Press, glfw::KeyA)   => self.position = self.position.sub_vec(&right.scalar_product(delta_time).scalar_product(SPEED)),
                _                               => {}
            }
        }
    }

    pub fn update(&mut self, 
        input_datas: &InputDatas) -> () {
        
        let delta_time = self.timer.delta_time();

        self.h_angle += MOUSE_SPEED * ::std::num::cast(delta_time).unwrap() * (self.window_size.x / 2f32 - input_datas.mouse_position.x);
        self.v_angle += MOUSE_SPEED * ::std::num::cast(delta_time).unwrap() * (self.window_size.y / 2f32 - input_datas.mouse_position.y);
        
        let dir     = Vec3::new(cos(self.v_angle) * sin(self.h_angle), sin(self.v_angle), cos(self.v_angle) * cos(self.h_angle));
        let right   = Vec3::new(sin(self.h_angle - CONST_RIGHT_ANGLE), 0f32, cos(self.h_angle - CONST_RIGHT_ANGLE));
        // let up      = right.cross_product(&dir);
        let up = Vec3::new(0f32, 1f32, 0f32);
        self.move(input_datas, &dir, &right, ::std::num::cast(delta_time).unwrap());
        self.mat_view = Mat4::look_at(&self.position, &self.position.add_vec(&dir), &up);
    }

    pub fn get_mat(&self) -> Mat4<f32> {
        self.mat_projection.cross_product(&self.mat_view)
    }
}
