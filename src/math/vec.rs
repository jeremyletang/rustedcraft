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

use std::num::sqrt;

#[deriving(Clone, Eq, Ord, Zero, ToStr)]
pub struct Vec4<T> {
    x: T,
    y: T,
    z: T,
    w: T
}

#[deriving(Clone, Eq, Ord, Zero, ToStr)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T
}

#[deriving(Clone, Eq, Ord, Zero, ToStr)]
pub struct Vec2<T> {
    x: T,
    y: T
}

impl<T: Float + Real> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }
}

impl<T: Float + Real> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn normalize(&mut self) -> () {
        let n = sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z));
        self.x = self.x / n;
        self.y = self.y / n;
        self.z = self.z / n;
    }

    pub fn cross_product(&self, oth: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: (self.y * oth.z) - (self.z * oth.y),
            y: (self.z * oth.x) - (self.x * oth.z),
            z: (self.x * oth.y) - (self.y * oth.x)
        }
    }

    pub fn dot_product(&self, oth: &Vec3<T>) -> T {
        (self.x * oth.x) + (self.y * oth.y) + (self.z * oth.z)
    }

    pub fn add_vec(&self, oth: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + oth.x,
            y: self.y + oth.y,
            z: self.z + oth.z
        }
    }

    pub fn sub_vec(&self, oth: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - oth.x,
            y: self.y - oth.y,
            z: self.z - oth.z
        }
    }

    pub fn scalar_product(&self, scalar: T) -> Vec3<T> {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}


impl<T: Float + Real> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 {
            x: x,
            y: y,
        }
    }

    pub fn normalize(&mut self) -> () {
        let n = sqrt((self.x * self.x) + (self.y * self.y));
        self.x = self.x / n;
        self.y = self.y / n;
    }

    pub fn dot_product(&self, oth: &Vec2<T>) -> T {
        (self.x * oth.x) + (self.y * oth.y)
    }

    pub fn add_vec(&self, oth: &Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + oth.x,
            y: self.y + oth.y
        }
    }

    pub fn sub_vec(&self, oth: &Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - oth.x,
            y: self.y - oth.y
        }
    }

    pub fn scalar_product(&self, scalar: T) -> Vec2<T> {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }

}



