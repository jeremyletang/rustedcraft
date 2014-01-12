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


pub use perf_metrics::fps::Fps;
pub use perf_metrics::mpf::Mpf;

use std::rc::Rc;
use std::cell::RefCell;

use text::Text;
use font::Font;
use math::Vec2;

pub mod fps;
pub mod mpf;

pub trait PerfHandler {
	fn reset(&mut self) -> ();
	fn get_value(&self) -> Option<f64>;
	fn frame_begin(&mut self) -> ();
	fn frame_end(&mut self) -> ();
}

pub struct PerfMetrics<T> {
	priv perf_handler: T,
	priv text: Text
}

impl<T: PerfHandler> PerfMetrics<T> {
	pub fn new(perf_handler: T, font: Rc<RefCell<Font>>) -> PerfMetrics<T> {
		PerfMetrics {
			perf_handler: 	perf_handler,
			text: 			Text::new(font, Some(~"Fps: "), Vec2::new(7f32, 7f32))
		}
	}

	pub fn frame_begin(&mut self) -> () {
		self.perf_handler.frame_begin();
	}

	pub fn frame_end(&mut self) -> () {
		self.perf_handler.frame_end();
		self.text.set_text(~"Fps: " + self.perf_handler.get_value().unwrap().to_str());
	}

	pub fn draw(&self) -> () {
		self.text.draw();
	}
}



