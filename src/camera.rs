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

use input_manager::InputDatas;
use math::{Mat4, Vec3};

pub static SPEED: f32        = 3f32;
pub static MOUSE_SPEED: f32  = 0.005f32;
pub static FOVY: f32         = 65f32;
pub static ASPECT: f32       = 4f32 / 3f32;
pub static Z_NEAR: f32       = 0.1f32;
pub static Z_FAR: f32        = 100f32;

pub struct Camera {
    priv mat_projection:    Mat4<f32>,
    priv mat_view:          Mat4<f32>,
    priv position:          Vec3<f32>,
    priv h_angle:           f32,
    priv v_angle:           f32
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            mat_projection:     Mat4::perspective(FOVY, ASPECT, Z_NEAR, Z_FAR),
            mat_view:           Mat4::<f32>::look_at(&Vec3::<f32>::new(10.,6.,-10.), &Vec3::<f32>::new(0.,0.,0.), &Vec3::<f32>::new(0.,1.,0.)),
            position:           Vec3::new(0f32, 0f32, 0f32),
            h_angle:            3.14f32,
            v_angle:            0f32
        }
    }

    pub fn update(&mut self, input_datas: &InputDatas) -> () {
    }

    pub fn get_mat(&self) -> Mat4<f32> {
        self.mat_projection.cross_product(&self.mat_view)
    }
}