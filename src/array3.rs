use std::ops::{Add, Div, Mul, Sub};

use num_traits::{Num, Zero};

use crate::{
    array1::Array1,
    array2::Array2,
    assert::{Assert, True},
    ops::{Concat, MatMul, ReshapeInner, SqueezeInner},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Array3<T, const A: usize, const B: usize, const C: usize> {
    data: [[[T; C]; B]; A],
}

impl<T, const A: usize, const B: usize, const C: usize> From<[[[T; C]; B]; A]>
    for Array3<T, A, B, C>
{
    fn from(data: [[[T; C]; B]; A]) -> Self {
        Self { data }
    }
}

impl<T: Num + Zero, const A: usize, const B: usize, const C: usize> Array3<T, A, B, C> {
    pub fn zeros() -> Self {
        let data: [[[T; C]; B]; A] =
            std::array::from_fn(|_| std::array::from_fn(|_| std::array::from_fn(|_| T::zero())));

        Self { data }
    }
}

impl<T, const A: usize, const B: usize, const C: usize> Add<T> for Array3<T, A, B, C>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let data: [[[T; C]; B]; A] = std::array::from_fn(|i| {
            std::array::from_fn(|j| std::array::from_fn(|k| self.data[i][j][k] + rhs))
        });

        Self { data }
    }
}

impl<T, const A: usize, const B: usize, const C: usize> Add for Array3<T, A, B, C>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let data: [[[T; C]; B]; A] = std::array::from_fn(|i| {
            std::array::from_fn(|j| std::array::from_fn(|k| self.data[i][j][k] + rhs.data[i][j][k]))
        });

        Self { data }
    }
}

impl<T, const A: usize, const B: usize, const C: usize> Sub<T> for Array3<T, A, B, C>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let data: [[[T; C]; B]; A] = std::array::from_fn(|i| {
            std::array::from_fn(|j| std::array::from_fn(|k| self.data[i][j][k] - rhs))
        });

        Self { data }
    }
}

impl<T, const A: usize, const B: usize, const C: usize> Sub for Array3<T, A, B, C>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let data: [[[T; C]; B]; A] = std::array::from_fn(|i| {
            std::array::from_fn(|j| std::array::from_fn(|k| self.data[i][j][k] - rhs.data[i][j][k]))
        });

        Self { data }
    }
}

impl<T, const A: usize, const B: usize, const C: usize> Mul<T> for Array3<T, A, B, C>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let data: [[[T; C]; B]; A] = std::array::from_fn(|i| {
            std::array::from_fn(|j| std::array::from_fn(|k| self.data[i][j][k] * rhs))
        });

        Self { data }
    }
}

impl<T, const A: usize, const B: usize, const C: usize> Div<T> for Array3<T, A, B, C>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let data: [[[T; C]; B]; A] = std::array::from_fn(|i| {
            std::array::from_fn(|j| std::array::from_fn(|k| self.data[i][j][k] / rhs))
        });

        Self { data }
    }
}

impl<T, const A: usize, const B: usize, const C: usize> Mul for Array3<T, A, B, C>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let data: [[[T; C]; B]; A] = std::array::from_fn(|i| {
            std::array::from_fn(|j| std::array::from_fn(|k| self.data[i][j][k] * rhs.data[i][j][k]))
        });

        Self { data }
    }
}

impl<T, const A: usize, const B: usize, const C: usize> Div for Array3<T, A, B, C>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let data: [[[T; C]; B]; A] = std::array::from_fn(|i| {
            std::array::from_fn(|j| std::array::from_fn(|k| self.data[i][j][k] / rhs.data[i][j][k]))
        });

        Self { data }
    }
}

impl<T, const A: usize, const B: usize, const C: usize, const D: usize>
    Concat<Array3<T, D, B, C>, Array3<T, { A + D }, B, C>> for Array3<T, A, B, C>
where
    T: Copy,
{
    fn concat(self, other: Array3<T, D, B, C>) -> Array3<T, { A + D }, B, C> {
        let data: [[[T; C]; B]; A + D] = std::array::from_fn(|i| {
            if i < A {
                self.data[i]
            } else {
                other.data[i - A]
            }
        });

        Array3::from(data)
    }
}
