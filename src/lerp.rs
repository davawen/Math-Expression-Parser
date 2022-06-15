use std::ops::{Add, Mul, Sub};

pub trait Lerp<I> {
    fn lerp(self, other: I, t: f64) -> I;

    fn inv_lerp(self, a: I, b: I) -> f64;

    fn lerp_map(self, a1: I, b1: I, a2: I, b2: I) -> I;
}

impl<T, I> Lerp<I> for T
where
    T: Copy,
    I: Copy + Add<Output = I> + Sub<Output = I> + Mul<f64, Output = I> + From<T>,
    f64: From<I>
{
    fn lerp(self, other: I, t: f64) -> I {
        I::from(self) + (other - I::from(self))*t
    }

    fn inv_lerp(self, a: I, b: I) -> f64 {
        f64::from( I::from(self) - a ) / f64::from( b - a )
    }

    fn lerp_map(self, a1: I, b1: I, a2: I, b2: I) -> I {
        a2.lerp(b2, self.inv_lerp(a1, b1))
    }
}
