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

pub struct Timer {
	priv last: 	f64
}

impl Timer {
	pub fn new() -> Timer {
		Timer {
			last: time::precise_time_s()
		}
	}

	pub fn current(&self) -> f64 {
		time::precise_time_s()
	}

	pub fn dela_time(&mut self) -> f64 {
		let ret_time = time::precise_time_s() - self.last;
		self.last = time::precise_time_s();
		ret_time
	}

	pub fn reset(&mut self) -> () {
		self.last = time::precise_time_s()
	}

	pub fn get_elapsed_time(&self) -> f64 {
		time::precise_time_s() - self.last
	}
}