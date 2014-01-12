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


#[crate_id = "rustedcraft#0.0.1"];
#[crate_type = "bin"];

#[warn(unnecessary_typecast)];
#[warn(non_uppercase_pattern_statics)];
#[warn(non_uppercase_statics)];
#[warn(non_camel_case_types)];

#[allow(dead_code)];

extern mod extra;
extern mod native;
extern mod gl;
extern mod glfw;
extern mod stb = "stb_image";

use game::Game;

mod glfw_utils;
mod game;
mod perf_metrics;
mod shaders;
mod math;
mod cube;
mod texture_loader;
mod world;
mod font;
mod text;
mod input_manager;

#[link(name = "glfw3")]
extern {}

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}


fn main() {
    do glfw::start {
        let mut game = Game::new();
        game.run();
    }
}
