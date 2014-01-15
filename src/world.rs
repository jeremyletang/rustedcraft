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

use math::{Mat4, Vec2};
use cube::Cube;
use texture_loader::TextureLoader;
use camera::Camera;
use input_manager::InputDatas;
use noise::perlin_noise::PerlinNoise;

pub struct CubeData {
    tex_id:         u32,
    position:   Mat4<f32>
}

pub struct World {
    priv cubes_datas:       ~[CubeData],
    // priv position:          Vec3<f32>,
    priv camera:            Camera,
    priv cube:              Cube,
    priv texture_loader:    Rc<RefCell<TextureLoader>>
}

impl World {
    pub fn new(texture_loader: Rc<RefCell<TextureLoader>>,
        window_size: Vec2<f32>) -> World {

        World {
            cubes_datas:        gen_world(),
            // position:           Vec3::new(0f32, 0f32, 0f32),
            camera:             Camera::new(window_size),
            cube:               Cube::new(),
            texture_loader:     texture_loader
        }
    }

    pub fn update(&mut self, 
        input_datas: &InputDatas) -> () {

        self.camera.update(input_datas);
        // self.position.x += move.x;
        // self.position.y += move.y;
        // self.position.z += move.z;
    }

    pub fn draw(&mut self) -> () {
        let mut mvp: Mat4<f32>;
        let mut model: Mat4<f32>;
        let cam = self.camera.get_mat();
        for c in self.cubes_datas.iter() {
            model = c.position.clone().cross_product(&Mat4::scale(0.5f32, 0.5f32, 0.5f32));
            // model.d1 += self.position.x;
            // model.d2 += self.position.y;
            // model.d3 += self.position.z;
            mvp = cam.cross_product(&model);
            self.cube.draw_cube(self.texture_loader.borrow().with(|loader| loader.get(c.tex_id)), &mvp);
        }
    }
}

// fn fbm(x: f32, y: f32, z: f32, octaves: i32, lacunarity: f32, gain: f32, noise: &PerlinNoise<f32>) -> f32 {
//     let mut amplitude = 1f32;
//     let mut frequency = 1f32;
//     let mut sum = 0f32;
//     for i in range(0, octaves) {
//         println!("SUM: {}", sum);
//         println!("AMPLITUDE *: {}", amplitude * (noise.noise(x * frequency.clone(), y * frequency.clone(), z * frequency.clone())));
//         sum += amplitude * (noise.noise(x * frequency.clone(), y * frequency.clone(), z * frequency.clone()));
//         amplitude *= gain;
//         frequency *= lacunarity;
//     }
//     sum
// }

// 16 / 10 / 16
fn gen_world() -> ~[CubeData] {
    let n = PerlinNoise::<f32>::new();
    let mut cubes_datas: ~[CubeData] = ~[];

    for y in range(0f32, 1f32) {
        for x in range(0f32, 80f32) {
            for z in range(0f32, 80f32) {
                // let tex = fbm(x * 0.01, y  * 0.1, z, 8, 2f32, 0.5f32, &n)* 0.5 + 0.5;
                let tex = n.noise(x * 0.01, y * 0.1 * 1.5, z * 0.05)  * 0.5 + 0.5;
                println!("{}", tex);
                // if (tex * 10f32) as u32 == 3 {
                    cubes_datas.push(CubeData { tex_id: 2 as u32 , position: Mat4::translate(x, (tex * 10f32).trunc(), z) });
                // }
            }
        }
    }

    cubes_datas
}
