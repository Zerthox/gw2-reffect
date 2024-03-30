use std::ops::{Add, Div, Mul, Sub};

pub trait ComponentWise<T>: Copy
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    fn component_wise(self, other: Self, op: impl Fn(T, T) -> T) -> Self;

    fn add(self, other: Self) -> Self {
        self.component_wise(other, Add::add)
    }

    fn sub(self, other: Self) -> Self {
        self.component_wise(other, Sub::sub)
    }

    fn mul(self, other: Self) -> Self {
        self.component_wise(other, Mul::mul)
    }

    fn div(self, other: Self) -> Self {
        self.component_wise(other, Div::div)
    }
}

impl ComponentWise<f32> for [f32; 2] {
    fn component_wise(self, other: Self, op: impl Fn(f32, f32) -> f32) -> Self {
        let [x1, y1] = self;
        let [x2, y2] = other;
        [op(x1, x2), op(y1, y2)]
    }
}

impl ComponentWise<f32> for [f32; 4] {
    fn component_wise(self, other: Self, op: impl Fn(f32, f32) -> f32) -> Self {
        let [a1, b1, c1, d1] = self;
        let [a2, b2, c2, d2] = other;
        [op(a1, a2), op(b1, b2), op(c1, c2), op(d1, d2)]
    }
}
