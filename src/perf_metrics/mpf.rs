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

extern mod extra;

use extra::time;
use perf_metrics::PerfHandler;

pub struct Mpf {
	priv m_sec: 		f64,
	priv frame_start: 	f64,
}

impl Mpf {
	pub fn new() -> Mpf {
		Mpf {
			m_sec: 		0.,
			frame_start: 	time::precise_time_s()
		}
	}
}

impl PerfHandler for Mpf {
	fn reset(&mut self) -> () {
		self.m_sec = 0.;
		self.frame_start = time::precise_time_s();
	}

	fn frame_begin(&mut self) -> () {
		self.frame_start = time::precise_time_s();
	}

	fn frame_end(&mut self) -> () {
		self.m_sec = time::precise_time_s() - self.frame_start;
	}

	fn get_value(&self) -> Option<f64> {
		Some(self.m_sec)
	}
}