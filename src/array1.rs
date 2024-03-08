use std::ops::{Add, Div, Mul, Sub};

use num_traits::{Num, Zero};

use crate::{
    array2::Array2,
    assert::{Assert, True},
    ops::{Concat, ReshapeInner, UnSqueezeInner},
};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Array1<T: Num, const A: usize> {
    pub(crate) data: [T; A],
}

impl<T: Num, const A: usize> From<[T; A]> for Array1<T, A> {
    fn from(data: [T; A]) -> Self {
        Self { data }
    }
}

impl<T: Num + Zero, const A: usize> Array1<T, A> {
    pub fn zeros() -> Self {
        let data: [T; A] = std::array::from_fn(|_| T::zero());

        Array1::from(data)
    }
}

impl<T, const A: usize> Add<T> for Array1<T, A>
where
    T: Num + Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let data: [T; A] = std::array::from_fn(|i| self.data[i] + rhs);

        Array1::from(data)
    }
}

impl<T, const A: usize> Add for Array1<T, A>
where
    T: Num + Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let data: [T; A] = std::array::from_fn(|i| self.data[i] + rhs.data[i]);

        Array1::from(data)
    }
}

impl<T, const A: usize> Sub<T> for Array1<T, A>
where
    T: Num + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let data: [T; A] = std::array::from_fn(|i| self.data[i] - rhs);

        Array1::from(data)
    }
}

impl<T, const A: usize> Sub for Array1<T, A>
where
    T: Num + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let data: [T; A] = std::array::from_fn(|i| self.data[i] - rhs.data[i]);

        Array1::from(data)
    }
}

impl<T, const A: usize> Mul<T> for Array1<T, A>
where
    T: Num + Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let data: [T; A] = std::array::from_fn(|i| self.data[i] * rhs);

        Array1::from(data)
    }
}

impl<T, const A: usize> Mul for Array1<T, A>
where
    T: Num + Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let data: [T; A] = std::array::from_fn(|i| self.data[i] * rhs.data[i]);

        Array1::from(data)
    }
}

impl<T, const A: usize> Div<T> for Array1<T, A>
where
    T: Num + Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let data: [T; A] = std::array::from_fn(|i| self.data[i] / rhs);

        Array1::from(data)
    }
}

impl<T, const A: usize> Div for Array1<T, A>
where
    T: Num + Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let data: [T; A] = std::array::from_fn(|i| self.data[i] / rhs.data[i]);

        Array1::from(data)
    }
}

impl<T, const A: usize, const B: usize> Concat<Array1<T, B>, Array1<T, { A + B }>> for Array1<T, A>
where
    T: Num + Copy,
    [T; A + B]:,
{
    fn concat(self, other: Array1<T, B>) -> Array1<T, { A + B }> {
        let mut data: [T; A + B] = [T::zero(); A + B];

        for (i, &x) in self.data.iter().chain(other.data.iter()).enumerate() {
            data[i] = x;
        }

        Array1::from(data)
    }
}

impl<T, const A: usize> UnSqueezeInner<Array2<T, 1, A>, 0> for Array1<T, A>
where
    T: Num + Copy,
{
    fn unsqueeze_(self) -> Array2<T, 1, A> {
        let data: [[T; A]; 1] = [self.data];

        Array2::from(data)
    }
}

impl<T, const A: usize> UnSqueezeInner<Array2<T, A, 1>, 1> for Array1<T, A>
where
    T: Num + Copy,
{
    fn unsqueeze_(self) -> Array2<T, A, 1> {
        let data: [[T; 1]; A] = std::array::from_fn(|i| [self.data[i]]);

        Array2::from(data)
    }
}

impl<T, const A: usize, const B: usize, const C: usize> ReshapeInner<Array2<T, B, C>>
    for Array1<T, A>
where
    T: Num + Copy,
    Assert<{ A == B * C }>: True,
{
    fn reshape_(self) -> Array2<T, B, C> {
        let data: [[T; C]; B] = std::array::from_fn(|i| {
            let mut row: [T; C] = [T::zero(); C];

            for (j, x) in row.iter_mut().enumerate() {
                *x = self.data[i * C + j];
            }

            row
        });

        Array2::from(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::{Reshape, UnSqueeze};

    #[test]
    fn test_array1_zeros() {
        let a: Array1<f64, 3> = Array1::zeros();

        assert_eq!(a.data, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_array1_from() {
        let a = Array1::from([1.0, 2.0, 3.0]);

        assert_eq!(a.data, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_array1_scalar_add() {
        let a: Array1<f64, 3> = Array1::zeros();
        let b = a + 1.0;

        assert_eq!(b.data, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_array1_add() {
        let a = Array1::from([1.0, 2.0, 3.0]);
        let b = Array1::from([1.0, 2.0, 3.0]);
        let c = a + b;

        assert_eq!(c.data, [2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_array1_scalar_sub() {
        let a: Array1<f64, 3> = Array1::zeros();
        let b = a - 1.0;

        assert_eq!(b.data, [-1.0, -1.0, -1.0]);
    }

    #[test]
    fn test_array1_sub() {
        let a = Array1::from([1.0, 2.0, 3.0]);
        let b = Array1::from([1.0, 2.0, 3.0]);
        let c = a - b;

        assert_eq!(c.data, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_array1_scalar_mul() {
        let a = Array1::from([1.0, 2.0, 3.0]);
        let b = a * 2.0;

        assert_eq!(b.data, [2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_array1_mul() {
        let a = Array1::from([1.0, 2.0, 3.0]);
        let b = Array1::from([1.0, 2.0, 3.0]);
        let c = a * b;

        assert_eq!(c.data, [1.0, 4.0, 9.0]);
    }

    #[test]
    fn test_array1_scalar_div() {
        let a = Array1::from([2.0, 4.0, 6.0]);
        let b = a / 2.0;

        assert_eq!(b.data, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_array1_div() {
        let a = Array1::from([1.0, 4.0, 9.0]);
        let b = Array1::from([1.0, 2.0, 3.0]);
        let c = a / b;

        assert_eq!(c.data, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_array1_concat() {
        let a = Array1::from([1.0, 2.0, 3.0]);
        let b = Array1::from([4.0, 5.0, 6.0]);
        let c = a.concat(b);

        assert_eq!(c.data, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_array1_unsqueeze_axis_0() {
        let a = Array1::from([1.0, 2.0, 3.0]);
        let b: Array2<f64, 1, 3> = a.unsqueeze::<_, 0>();

        assert_eq!(b, Array2::from([[1.0, 2.0, 3.0]]));
    }

    #[test]
    fn test_array1_unsqueeze_axis_1() {
        let a = Array1::from([1.0, 2.0, 3.0]);
        let b = a.unsqueeze::<_, 1>();

        assert_eq!(b, Array2::from([[1.0], [2.0], [3.0]]));
    }

    #[test]
    fn test_array1_reshape_array2_inner() {
        let a = Array1::from([1.0, 2.0, 3.0]);
        let b: Array2<f64, 3, 1> = a.reshape();

        assert_eq!(b, Array2::from([[1.0], [2.0], [3.0]]));
    }

    #[test]
    fn test_array1_reshape_array2_outer() {
        let a = Array1::from([1.0, 2.0, 3.0]);
        let b: Array2<_, 1, 3> = a.reshape();

        assert_eq!(b, Array2::from([[1.0, 2.0, 3.0]]));
    }
}
