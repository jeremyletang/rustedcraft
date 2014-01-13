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

use extra::time;
use perf_metrics::PerfHandler;

pub struct Fps {
	priv frames: 		f64,
	priv cur_frames: 	f64,
	priv frame_start: 	f64,
}

impl Fps {
	pub fn new() -> Fps {
		Fps {
			frames: 		0.,
			cur_frames: 	0.,
			frame_start: 	time::precise_time_s()
		}
	}
}

impl PerfHandler for Fps {
	fn reset(&mut self) -> () {
		self.frames = 0.;
		self.cur_frames = 0.;
		self.frame_start = time::precise_time_s();
	}

	fn frame_begin(&mut self) -> () {

	}

	fn frame_end(&mut self) -> () {
		self.cur_frames += 1.;
		if time::precise_time_s() - self.frame_start >= 1. {
			self.frame_start = time::precise_time_s();
			self.frames = self.cur_frames;
			self.cur_frames = 0.;
		}
	}

	fn get_value(&self) -> Option<f64> {
		Some(self.frames)
	}
}