#![allow(unused)]

use std::{
    borrow::Borrow,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub trait ComponentWise<T>: Sized
where
    T: Copy
        + Neg<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    /// Performs a component-wise map operation.
    fn component_wise_map(&self, op: impl Fn(T) -> T) -> Self;

    /// Performs a component-wise zip operation.
    fn component_wise_zip(&self, other: &Self, op: impl Fn(T, T) -> T) -> Self;

    /// Computes component-wise negation.
    fn neg(&self) -> Self {
        self.component_wise_map(Neg::neg)
    }

    /// Computes component-wise addition.
    fn add(&self, other: impl Borrow<Self>) -> Self {
        self.component_wise_zip(other.borrow(), Add::add)
    }

    /// Computes component-wise addition with a scalar value.
    fn add_scalar(&self, scalar: T) -> Self {
        self.component_wise_map(|el| el + scalar)
    }

    /// Computes component-wise subtraction.
    fn sub(&self, other: impl Borrow<Self>) -> Self {
        self.component_wise_zip(other.borrow(), Sub::sub)
    }

    /// Computes component-wise subtraction with a scalar value.
    fn sub_scalar(&self, scalar: T) -> Self {
        self.component_wise_map(|el| el - scalar)
    }

    /// Computes component-wise multiplication.
    fn mul(&self, other: impl Borrow<Self>) -> Self {
        self.component_wise_zip(other.borrow(), Mul::mul)
    }

    /// Computes component-wise multiplication with a scalar value.
    fn mul_scalar(&self, scalar: T) -> Self {
        self.component_wise_map(|el| el * scalar)
    }

    /// Computes component-wise division.
    fn div(&self, other: impl Borrow<Self>) -> Self {
        self.component_wise_zip(other.borrow(), Div::div)
    }

    /// Computes component-wise division with a scalar value.
    fn div_scalar(&self, scalar: T) -> Self {
        self.component_wise_map(|el| el / scalar)
    }
}

impl<T> ComponentWise<T> for [T; 2]
where
    T: Copy
        + Neg<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    fn component_wise_map(&self, op: impl Fn(T) -> T) -> Self {
        let [x, y] = *self;
        [op(x), op(y)]
    }

    fn component_wise_zip(&self, other: &Self, op: impl Fn(T, T) -> T) -> Self {
        let [x1, y1] = *self;
        let [x2, y2] = *other;
        [op(x1, x2), op(y1, y2)]
    }
}

impl<T> ComponentWise<T> for [T; 4]
where
    T: Copy
        + Neg<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    fn component_wise_map(&self, op: impl Fn(T) -> T) -> Self {
        let [x, y, z, w] = *self;
        [op(x), op(y), op(z), op(w)]
    }

    fn component_wise_zip(&self, other: &Self, op: impl Fn(T, T) -> T) -> Self {
        let [x1, y1, z1, w1] = *self;
        let [x2, y2, z2, w2] = *other;
        [op(x1, x2), op(y1, y2), op(z1, z2), op(w1, w2)]
    }
}
