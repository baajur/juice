// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Vector operations.
use crate::vector::ops::{Asum, Axpy, Copy, Dot, Iamax, Nrm2, Scal};
use num::traits::NumCast;
use num_complex::{Complex32, Complex64};

pub mod ll;
pub mod ops;

/// Methods that allow a type to be used in BLAS functions as a vector.
pub trait Vector<T> {
    /// The stride within the vector. For example, if `inc` returns 7, every
    /// 7th element is used. Defaults to 1.
    fn inc(&self) -> u32 {
        1
    }
    /// The number of elements in the vector.
    fn len(&self) -> u32;
    /// An unsafe pointer to a contiguous block of memory.
    fn as_ptr(&self) -> *const T;
    /// An unsafe mutable pointer to a contiguous block of memory.
    fn as_mut_ptr(&mut self) -> *mut T;
}

impl<'a, T> Into<Vec<T>> for &'a dyn Vector<T>
where
    T: Copy,
{
    fn into(self) -> Vec<T> {
        let n = self.len() as usize;

        let mut x = Vec::with_capacity(n);
        unsafe {
            x.set_len(n);
        }
        Copy::copy(self, &mut x);

        x
    }
}

pub trait VectorOperations<T>: Sized + Vector<T>
where
    T: Copy + Axpy + Scal + Dot + Nrm2 + Asum + Iamax,
{
    fn update(&mut self, alpha: &T, x: &dyn Vector<T>) -> &mut Self {
        Axpy::axpy(alpha, x, self);
        self
    }

    fn scale(&mut self, alpha: &T) -> &mut Self {
        Scal::scal(alpha, self);
        self
    }

    fn dot(&self, x: &dyn Vector<T>) -> T {
        Dot::dot(self, x)
    }

    fn abs_sum(&self) -> T {
        Asum::asum(self)
    }

    fn norm(&self) -> T {
        Nrm2::nrm2(self)
    }

    fn max_index(&self) -> usize {
        Iamax::iamax(self)
    }
}

impl<T> Vector<T> for Vec<T> {
    fn len(&self) -> u32 {
        let l: Option<u32> = NumCast::from(Vec::len(self));
        match l {
            Some(l) => l,
            None => panic!(),
        }
    }

    fn as_ptr(&self) -> *const T {
        self[..].as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        (&mut self[..]).as_mut_ptr()
    }
}

impl<T> Vector<T> for [T] {
    fn len(&self) -> u32 {
        let l: Option<u32> = NumCast::from(<[T]>::len(self));
        match l {
            Some(l) => l,
            None => panic!(),
        }
    }

    fn as_ptr(&self) -> *const T {
        <[T]>::as_ptr(self)
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        <[T]>::as_mut_ptr(self)
    }
}

macro_rules! operations_impl(
    ($v: ident, $($t: ty), +) => (
        $( impl VectorOperations<$t> for $v<$t> {} )+
    )
);

operations_impl!(Vec, f32, f64, Complex32, Complex64);
//impl<'a> VectorOperations<f32> for &'a [f32] {}
//impl<'a> VectorOperations<f64> for &'a [f64] {}
//impl<'a> VectorOperations<Complex32> for &'a [Complex32] {}
//impl<'a> VectorOperations<Complex64> for &'a [Complex64] {}
