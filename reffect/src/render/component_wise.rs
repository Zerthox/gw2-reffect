#![allow(unused)]

use std::{
    borrow::Borrow,
    ops::{Add, Div, Mul, Sub},
};

pub trait ComponentWise<T>: Sized
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn component_wise_map(&self, op: impl Fn(T) -> T) -> Self;

    fn component_wise_zip(&self, other: &Self, op: impl Fn(T, T) -> T) -> Self;

    fn add(&self, other: impl Borrow<Self>) -> Self {
        self.component_wise_zip(other.borrow(), Add::add)
    }

    fn add_scalar(&self, scalar: T) -> Self {
        self.component_wise_map(|el| el + scalar)
    }

    fn sub(&self, other: impl Borrow<Self>) -> Self {
        self.component_wise_zip(other.borrow(), Sub::sub)
    }

    fn sub_scalar(&self, scalar: T) -> Self {
        self.component_wise_map(|el| el - scalar)
    }

    fn mul(&self, other: impl Borrow<Self>) -> Self {
        self.component_wise_zip(other.borrow(), Mul::mul)
    }

    fn mul_scalar(&self, scalar: T) -> Self {
        self.component_wise_map(|el| el * scalar)
    }

    fn div(&self, other: impl Borrow<Self>) -> Self {
        self.component_wise_zip(other.borrow(), Div::div)
    }

    fn div_scalar(&self, scalar: T) -> Self {
        self.component_wise_map(|el| el / scalar)
    }
}

impl<T> ComponentWise<T> for [T; 2]
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn component_wise_map(&self, op: impl Fn(T) -> T) -> Self {
        self.map(op)
    }

    fn component_wise_zip(&self, other: &Self, op: impl Fn(T, T) -> T) -> Self {
        let [x1, y1] = *self;
        let [x2, y2] = *other;
        [op(x1, x2), op(y1, y2)]
    }
}

impl<T> ComponentWise<T> for [T; 4]
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn component_wise_map(&self, op: impl Fn(T) -> T) -> Self {
        self.map(op)
    }

    fn component_wise_zip(&self, other: &Self, op: impl Fn(T, T) -> T) -> Self {
        let [a1, b1, c1, d1] = *self;
        let [a2, b2, c2, d2] = *other;
        [op(a1, a2), op(b1, b2), op(c1, c2), op(d1, d2)]
    }
}
