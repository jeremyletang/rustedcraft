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

use math::{Mat4, Vec3};
use cube::Cube;
use texture_loader::TextureLoader;

pub struct CubeData {
    tex_id:         u32,
    position:   Mat4<f32>
}

pub struct World {
    priv cubes_datas:       ~[CubeData],
    priv position:          Vec3<f32>,
    priv mat_projection:    Mat4<f32>,
    priv mat_view:          Mat4<f32>,
    priv cube:              Cube,
    priv texture_loader:    Rc<RefCell<TextureLoader>>
}

impl World {
    pub fn new(texture_loader: Rc<RefCell<TextureLoader>>) -> World {
        World {
            cubes_datas:        gen_world(),
            position:           Vec3::new(0f32, 0f32, 0f32),
            mat_projection:     Mat4::perspective(65f32, 4f32 / 3f32, 0.1f32, 100f32),
            mat_view:           Mat4::<f32>::look_at(&Vec3::<f32>::new(10.,6.,-10.), &Vec3::<f32>::new(0.,0.,0.), &Vec3::<f32>::new(0.,1.,0.)),
            cube:               Cube::new(),
            texture_loader:     texture_loader
        }
    }

    pub fn update_position(&mut self, move: Vec3<f32>) -> () {
        self.position.x += move.x;
        self.position.y += move.y;
        self.position.z += move.z;
    }

    pub fn draw(&mut self) -> () {
        let mut mvp: Mat4<f32>;
        let mut model: Mat4<f32>;
        for c in self.cubes_datas.iter() {
            model = c.position.clone();
            model.d1 += self.position.x;
            model.d2 += self.position.y;
            model.d3 += self.position.z;
            mvp = self.mat_projection.cross_product(&self.mat_view).cross_product(&model);
            self.cube.draw_cube(self.texture_loader.borrow().with(|loader| loader.get(c.tex_id)), &mvp);
        }
    }
}

fn gen_world() -> ~[CubeData] {
    let mut cubes_datas: ~[CubeData] = ~[];
    cubes_datas.push(CubeData { tex_id: 1, position: Mat4::identity() });
    cubes_datas.push(CubeData { tex_id: 1, position: Mat4::translate(2f32, 0f32, 0f32) });
    cubes_datas.push(CubeData { tex_id: 1, position: Mat4::translate(2f32, 2f32, 0f32) });
    cubes_datas.push(CubeData { tex_id: 1, position: Mat4::translate(0f32, 2f32, 0f32) });
    cubes_datas.push(CubeData { tex_id: 1, position: Mat4::translate(2f32, 0f32, -2f32) });
    cubes_datas.push(CubeData { tex_id: 2, position: Mat4::translate(2f32, 2f32, -2f32) });
    cubes_datas.push(CubeData { tex_id: 1, position: Mat4::translate(0f32, 2f32, -2f32) });
    cubes_datas.push(CubeData { tex_id: 1, position: Mat4::translate(0f32, 0f32, -2f32) });
    cubes_datas
}
