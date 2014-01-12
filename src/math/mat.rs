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

use std::num::{Zero, zero, one, cast, tan, Real, cos, sin};

use math::vec::Vec3;

#[deriving(Clone, Eq, Ord, Zero, ToStr)]
pub struct Mat4<T> {
    a1: T, a2: T, a3: T, a4: T,
    b1: T, b2: T, b3: T, b4: T,
    c1: T, c2: T, c3: T, c4: T,
    d1: T, d2: T, d3: T, d4: T, 
}

impl<T: Float + Real> Mat4<T> {
    pub fn identity() -> Mat4<T> {
        Mat4 {
            a1: one(),  a2: zero(), a3: zero(), a4: zero(),
            b1: zero(), b2: one(),  b3: zero(), b4: zero(),
            c1: zero(), c2: zero(), c3: one(),  c4: zero(),
            d1: zero(), d2: zero(), d3: zero(), d4: one(),  
        }
    }

    pub fn translate(x: T, y: T, z: T) -> Mat4<T> {
        Mat4 {
            a1: one(),  a2: zero(), a3: zero(), a4: zero(),
            b1: zero(), b2: one(),  b3: zero(), b4: zero(),
            c1: zero(), c2: zero(), c3: one(),  c4: zero(),
            d1: x,      d2: y,      d3: z,      d4: one(),  
        }
    }

    pub fn scale(x: T, y: T, z: T) -> Mat4<T> {
        Mat4{
            a1: x,      a2: zero(), a3: zero(), a4: zero(),
            b1: zero(), b2: y,      b3: zero(), b4: zero(),
            c1: zero(), c2: zero(), c3: z,      c4: zero(),
            d1: zero(), d2: zero(), d3: zero(), d4: one(),  
        }
    }

    pub fn rotate(x: T, y: T, z: T) -> Mat4<T> {
        let mut ret_mat = Mat4::identity();
        let a   = cos(x.clone());
        let b   = sin(x.clone());
        let c   = cos(y.clone());
        let d   = sin(y.clone());
        let e   = cos(z.clone());
        let f   = sin(z.clone());
        let ad  = a * d;
        let bd  = b * d;

        ret_mat.a1 = c * e;
        ret_mat.a2 = -c * f;
        ret_mat.a3 = -d;
        ret_mat.b1 = -bd * e + a * f;
        ret_mat.b2 = bd * f + a * e;
        ret_mat.b3 = -b * c;
        ret_mat.c1 = ad * e + b * f;
        ret_mat.c2 = -ad * f + b * e;
        ret_mat.c3 = a * c;

        ret_mat
    }

    pub fn perspective(fovy: T, aspect: T, z_near: T, z_far: T) -> Mat4<T> {
        let right: T = z_near * tan(fovy * cast::<f32, T>(::std::f32::consts::PI).unwrap() / cast::<f32, T>(360.0).unwrap());
        let top: T = right * aspect;
        Mat4::frustrum(-right, right, -top, top, z_near, z_far)
    }

    fn frustrum(bottom: T, 
                top: T, 
                left: T, 
                right: T, 
                near_val: T, 
                far_val: T) -> Mat4<T> {

        Mat4 {
            a1: (cast::<f32, T>(2.).unwrap() * near_val) / (right - left), 
            a2: zero(), 
            a3: zero(),
            a4: zero(), 
            b1: zero(), 
            b2: (cast::<f32, T>(2.).unwrap() * near_val) / (top - bottom), 
            b3: zero(), 
            b4: zero(), 
            c1: (right + left) / (right - left), 
            c2: (top + bottom) / (top - bottom), 
            c3: -((far_val + near_val) / (far_val - near_val)), 
            c4: -one::<T>(), 
            d1: zero(),
            d2: zero(),
            d3: -((cast::<f32, T>(2.).unwrap() * far_val * near_val) / (far_val - near_val)),
            d4: zero()
        }
    }

    pub fn look_at(eye_position: &Vec3<T>, center: &Vec3<T>, up_vector: &Vec3<T>) -> Mat4<T> {
        let mut forward = Vec3::<T>::new(center.x - eye_position.x,
                                         center.y - eye_position.y,
                                         center.z - eye_position.z);
        forward.normalize();
        let mut side = forward.cross_product(up_vector);
        side.normalize();
        let up: Vec3<T> = side.cross_product(&forward);

        Mat4 {
            a1: side.x.clone(), 
            a2: up.x.clone(), 
            a3: -forward.x.clone(), 
            a4: zero(),
            b1: side.y.clone(), 
            b2: up.y.clone(), 
            b3: -forward.y.clone(), 
            b4: zero(),
            c1: side.z.clone(), 
            c2: up.z.clone(), 
            c3: -forward.z.clone(), 
            c4: zero(),
            d1: -eye_position.scalar_product(&side),
            d2: -eye_position.scalar_product(&up), 
            d3: eye_position.scalar_product(&forward), 
            d4: one(),
        }
    }

    pub fn cross_product(&self, oth: &Mat4<T>) -> Mat4<T> {
        Mat4 {
            a1: (self.a1 * oth.a1) + (self.b1 * oth.a2) + (self.c1 * oth.a3) + (self.d1 * oth.a4), 
            a2: (self.a2 * oth.a1) + (self.b2 * oth.a2) + (self.c2 * oth.a3) + (self.d2 * oth.a4), 
            a3: (self.a3 * oth.a1) + (self.b3 * oth.a2) + (self.c3 * oth.a3) + (self.d3 * oth.a4), 
            a4: (self.a4 * oth.a1) + (self.b4 * oth.a2) + (self.c4 * oth.a3) + (self.d4 * oth.a4), 
            b1: (self.a1 * oth.b1) + (self.b1 * oth.b2) + (self.c1 * oth.b3) + (self.d1 * oth.b4), 
            b2: (self.a2 * oth.b1) + (self.b2 * oth.b2) + (self.c2 * oth.b3) + (self.d2 * oth.b4), 
            b3: (self.a3 * oth.b1) + (self.b3 * oth.b2) + (self.c3 * oth.b3) + (self.d3 * oth.b4), 
            b4: (self.a4 * oth.b1) + (self.b4 * oth.b2) + (self.c4 * oth.b3) + (self.d4 * oth.b4), 
            c1: (self.a1 * oth.c1) + (self.b1 * oth.c2) + (self.c1 * oth.c3) + (self.d1 * oth.c4), 
            c2: (self.a2 * oth.c1) + (self.b2 * oth.c2) + (self.c2 * oth.c3) + (self.d2 * oth.c4), 
            c3: (self.a3 * oth.c1) + (self.b3 * oth.c2) + (self.c3 * oth.c3) + (self.d3 * oth.c4), 
            c4: (self.a4 * oth.c1) + (self.b4 * oth.c2) + (self.c4 * oth.c3) + (self.d4 * oth.c4), 
            d1: (self.a1 * oth.d1) + (self.b1 * oth.d2) + (self.c1 * oth.d3) + (self.d1 * oth.d4),          
            d2: (self.a2 * oth.d1) + (self.b2 * oth.d2) + (self.c2 * oth.d3) + (self.d2 * oth.d4),
            d3: (self.a3 * oth.d1) + (self.b3 * oth.d2) + (self.c3 * oth.d3) + (self.d3 * oth.d4),
            d4: (self.a4 * oth.d1) + (self.b4 * oth.d2) + (self.c4 * oth.d3) + (self.d4 * oth.d4),
        }
    }
}

/*
    void glhLookAtf2(float *matrix, float *eyePosition3D,
                     float *center3D, float *upVector3D )
    {
        float forward[3], side[3], up[3];
        float matrix2[16], resultMatrix[16];
        //------------------
        forward[0] = center3D[0] - eyePosition3D[0];
        forward[1] = center3D[1] - eyePosition3D[1];
        forward[2] = center3D[2] - eyePosition3D[2];
        NormalizeVector(forward);
        //------------------
        //Side = forward x up
        ComputeNormalOfPlane(side, forward, upVector3D);
        NormalizeVector(side);
        //------------------
        //Recompute up as: up = side x forward
        ComputeNormalOfPlane(up, side, forward);
        //------------------
        matrix2[0] = side[0];
        matrix2[4] = side[1];
        matrix2[8] = side[2];
        matrix2[12] = 0.0;
        //------------------
        matrix2[1] = up[0];
        matrix2[5] = up[1];
        matrix2[9] = up[2];
        matrix2[13] = 0.0;
        //------------------
        matrix2[2] = -forward[0];
        matrix2[6] = -forward[1];
        matrix2[10] = -forward[2];
        matrix2[14] = 0.0;
        //------------------
        matrix2[3] = matrix2[7] = matrix2[11] = 0.0;
        matrix2[15] = 1.0;
        //------------------
        MultiplyMatrices4by4OpenGL_FLOAT(resultMatrix, matrix, matrix2);
        glhTranslatef2(resultMatrix,
                      -eyePosition3D[0], -eyePosition3D[1], -eyePosition3D[2]);
        //------------------
        memcpy(matrix, resultMatrix, 16*sizeof(float));
    }
*/

