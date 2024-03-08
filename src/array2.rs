use std::ops::{Add, Div, Mul, Sub};

use num_traits::{Num, Zero};

use crate::{
    array1::Array1,
    assert::{Assert, True},
    ops::{Concat, MatMul, ReshapeInner, SqueezeInner},
};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Array2<T, const A: usize, const B: usize> {
    pub(crate) data: [[T; B]; A],
}

impl<T, const A: usize, const B: usize> From<[[T; B]; A]> for Array2<T, A, B> {
    fn from(data: [[T; B]; A]) -> Self {
        Self { data }
    }
}

impl<T: Num + Zero, const A: usize, const B: usize> Array2<T, A, B> {
    pub fn zeros() -> Self {
        let data: [[T; B]; A] = std::array::from_fn(|_| std::array::from_fn(|_| T::zero()));

        Self { data }
    }
}

impl<T, const A: usize, const B: usize> Add<T> for Array2<T, A, B>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let data: [[T; B]; A] =
            std::array::from_fn(|i| std::array::from_fn(|j| self.data[i][j] + rhs));

        Self { data }
    }
}

impl<T, const A: usize, const B: usize> Add for Array2<T, A, B>
where
    T: Num + Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let data: [[T; B]; A] =
            std::array::from_fn(|i| std::array::from_fn(|j| self.data[i][j] + rhs.data[i][j]));

        Self { data }
    }
}

impl<T, const A: usize, const B: usize> Sub<T> for Array2<T, A, B>
where
    T: Num + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let data: [[T; B]; A] =
            std::array::from_fn(|i| std::array::from_fn(|j| self.data[i][j] - rhs));

        Self { data }
    }
}

impl<T, const A: usize, const B: usize> Sub for Array2<T, A, B>
where
    T: Num + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let data: [[T; B]; A] =
            std::array::from_fn(|i| std::array::from_fn(|j| self.data[i][j] - rhs.data[i][j]));

        Self { data }
    }
}

impl<T, const A: usize, const B: usize> Mul<T> for Array2<T, A, B>
where
    T: Num + Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let data: [[T; B]; A] =
            std::array::from_fn(|i| std::array::from_fn(|j| self.data[i][j] * rhs));

        Self { data }
    }
}

impl<T, const A: usize, const B: usize> Div<T> for Array2<T, A, B>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let data: [[T; B]; A] =
            std::array::from_fn(|i| std::array::from_fn(|j| self.data[i][j] / rhs));

        Self { data }
    }
}

impl<T, const A: usize, const B: usize> Mul for Array2<T, A, B>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let data: [[T; B]; A] =
            std::array::from_fn(|i| std::array::from_fn(|j| self.data[i][j] * rhs.data[i][j]));

        Self { data }
    }
}

impl<T, const A: usize, const B: usize> Div for Array2<T, A, B>
where
    T: Num + Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let data: [[T; B]; A] =
            std::array::from_fn(|i| std::array::from_fn(|j| self.data[i][j] / rhs.data[i][j]));

        Self { data }
    }
}

impl<T, const A: usize, const B: usize, const C: usize>
    Concat<Array2<T, C, B>, Array2<T, { A + C }, B>> for Array2<T, A, B>
where
    T: Copy,
    [(); A + C]:,
{
    fn concat(self, other: Array2<T, C, B>) -> Array2<T, { A + C }, B> {
        let data: [[T; B]; A + C] = std::array::from_fn(|i| {
            if i < A {
                self.data[i]
            } else {
                other.data[i - A]
            }
        });

        Array2::from(data)
    }
}

impl<T, const A: usize, const B: usize, const C: usize>
    Concat<Array2<T, A, C>, Array2<T, A, { B + C }>> for Array2<T, A, B>
where
    T: Num + Copy,
    [(); B + C]:,
{
    fn concat(self, other: Array2<T, A, C>) -> Array2<T, A, { B + C }> {
        let data: [[T; B + C]; A] = std::array::from_fn(|i| {
            let mut row = [T::zero(); B + C];
            row[..B].copy_from_slice(&self.data[i]);
            row[B..].copy_from_slice(&other.data[i]);
            row
        });

        Array2::from(data)
    }
}

impl<T, const A: usize, const B: usize> SqueezeInner<Array1<T, A>, 0> for Array2<T, 1, B>
where
    T: Num + Copy,
{
    fn squeeze_(self) -> Array1<T, A> {
        let data: [T; A] = std::array::from_fn(|i| self.data[0][i]);

        Array1::from(data)
    }
}

impl<T, const A: usize, const B: usize> SqueezeInner<Array1<T, B>, 1> for Array2<T, A, 1>
where
    T: Num + Copy,
{
    fn squeeze_(self) -> Array1<T, B> {
        let data: [T; B] = std::array::from_fn(|i| self.data[i][0]);

        Array1::from(data)
    }
}

impl<T, const A: usize, const B: usize, const C: usize> MatMul<Array2<T, B, C>, Array2<T, A, C>>
    for Array2<T, A, B>
where
    T: Num + Copy,
{
    fn matmul(self, rhs: Array2<T, B, C>) -> Array2<T, A, C> {
        let data: [[T; C]; A] = std::array::from_fn(|j| {
            std::array::from_fn(|i| {
                let mut sum = T::zero();
                for k in 0..B {
                    sum = sum + self.data[j][k] * rhs.data[k][i];
                }
                sum
            })
        });

        Array2::from(data)
    }
}

impl<T, const A: usize, const B: usize> MatMul<Array1<T, B>, Array1<T, A>> for Array2<T, A, B>
where
    T: Num + Copy,
{
    fn matmul(self, rhs: Array1<T, B>) -> Array1<T, A> {
        let data: [T; A] = std::array::from_fn(|i| {
            let mut sum = T::zero();
            for j in 0..B {
                sum = sum + self.data[i][j] * rhs.data[j];
            }
            sum
        });

        Array1::from(data)
    }
}

impl<T, const A: usize, const B: usize, const C: usize> ReshapeInner<Array1<T, C>>
    for Array2<T, A, B>
where
    T: Num + Copy,
    Assert<{ A * B == C }>: True,
{
    fn reshape_(self) -> Array1<T, C> {
        let data = std::array::from_fn(|i| self.data[i / B][i % B]);

        Array1::from(data)
    }
}

impl<T, const A: usize, const B: usize, const C: usize, const D: usize>
    ReshapeInner<Array2<T, C, D>> for Array2<T, A, B>
where
    T: Num + Copy,
    Assert<{ A * B == C * D }>: True,
{
    fn reshape_(self) -> Array2<T, C, D> {
        let data: [[T; D]; C] = std::array::from_fn(|i| {
            std::array::from_fn(|j| {
                let pos = i * D + j;
                self.data[pos / B][pos % B]
            })
        });

        Array2::from(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::{Reshape, Squeeze};

    #[test]
    fn test_array2_zeros() {
        let a = Array2::<f64, 2, 3>::zeros();
        let b = [[0.0; 3]; 2];

        assert_eq!(a.data, b);
    }

    #[test]
    fn test_array2_from() {
        let a = Array2::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];

        assert_eq!(a.data, b);
    }

    #[test]
    fn test_array2_scalar_add() {
        let a = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b = Array2::<f64, 2, 3>::from([[2.0, 3.0, 4.0], [5.0, 6.0, 7.0]]);

        assert_eq!(a + 1.0, b);
    }

    #[test]
    fn test_array2_add() {
        let a = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b = Array2::<f64, 2, 3>::from([[2.0, 3.0, 4.0], [5.0, 6.0, 7.0]]);
        let c = Array2::<f64, 2, 3>::from([[3.0, 5.0, 7.0], [9.0, 11.0, 13.0]]);

        assert_eq!(a + b, c);
    }

    #[test]
    fn test_array2_scalar_sub() {
        let a = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b = Array2::<f64, 2, 3>::from([[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]);

        assert_eq!(a - 1.0, b);
    }

    #[test]
    fn test_array2_sub() {
        let a = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b = Array2::<f64, 2, 3>::from([[2.0, 3.0, 4.0], [5.0, 6.0, 7.0]]);
        let c = Array2::<f64, 2, 3>::from([[-1.0, -1.0, -1.0], [-1.0, -1.0, -1.0]]);

        assert_eq!(a - b, c);
    }

    #[test]
    fn test_array2_scalar_mul() {
        let a = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b = Array2::<f64, 2, 3>::from([[2.0, 4.0, 6.0], [8.0, 10.0, 12.0]]);

        assert_eq!(a * 2.0, b);
    }

    #[test]
    fn test_array2_mul() {
        let a = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b = Array2::<f64, 2, 3>::from([[2.0, 3.0, 4.0], [5.0, 6.0, 7.0]]);
        let c = Array2::<f64, 2, 3>::from([[2.0, 6.0, 12.0], [20.0, 30.0, 42.0]]);

        assert_eq!(a * b, c);
    }

    #[test]
    fn test_array2_scalar_div() {
        let a = Array2::<f64, 2, 3>::from([[2.0, 4.0, 6.0], [8.0, 10.0, 12.0]]);
        let b = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);

        assert_eq!(a / 2.0, b);
    }

    #[test]
    fn test_array2_div() {
        let a = Array2::<f64, 2, 3>::from([[2.0, 6.0, 12.0], [20.0, 30.0, 42.0]]);
        let b = Array2::<f64, 2, 3>::from([[2.0, 3.0, 4.0], [5.0, 6.0, 7.0]]);
        let c = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);

        assert_eq!(a / b, c);
    }

    #[test]
    fn test_array2_concat_axis_0() {
        let a = Array2::<f64, 3, 2>::from([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);
        let b = Array2::<f64, 4, 2>::from([[7.0, 8.0], [9.0, 10.0], [11.0, 12.0], [13.0, 14.0]]);

        let c = Array2::<f64, 7, 2>::from([
            [1.0, 2.0],
            [3.0, 4.0],
            [5.0, 6.0],
            [7.0, 8.0],
            [9.0, 10.0],
            [11.0, 12.0],
            [13.0, 14.0],
        ]);

        assert_eq!(a.concat(b), c);
    }

    #[test]
    fn test_array2_concat_axis_1() {
        let a = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b = Array2::<f64, 2, 4>::from([[7.0, 8.0, 9.0, 10.0], [11.0, 12.0, 13.0, 14.0]]);

        let c = Array2::<f64, 2, 7>::from([
            [1.0, 2.0, 3.0, 7.0, 8.0, 9.0, 10.0],
            [4.0, 5.0, 6.0, 11.0, 12.0, 13.0, 14.0],
        ]);

        assert_eq!(a.concat(b), c);
    }

    #[test]
    fn test_array2_concat_same_dims_axis_0() {
        let a = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b = Array2::<f64, 2, 3>::from([[7.0, 8.0, 9.0], [10.0, 11.0, 12.0]]);

        let expected = Array2::<f64, 4, 3>::from([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
            [10.0, 11.0, 12.0],
        ]);

        let actual: Array2<f64, 4, 3> = a.concat(b);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_array2_concat_same_dims_axis_1() {
        let a = Array2::<f64, 3, 2>::from([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);
        let b = Array2::<f64, 3, 2>::from([[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]);

        let expected = Array2::<f64, 3, 4>::from([
            [1.0, 2.0, 7.0, 8.0],
            [3.0, 4.0, 9.0, 10.0],
            [5.0, 6.0, 11.0, 12.0],
        ]);

        let actual: Array2<f64, 3, 4> = a.concat(b);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_array2_squeeze_axis_0() {
        let a = Array2::<f64, 1, 3>::from([[1.0, 2.0, 3.0]]);
        let b = a.squeeze::<_, 0>();

        assert_eq!(b, Array1::from([1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_array2_squeeze_axis_1() {
        let a = Array2::<f64, 3, 1>::from([[1.0], [2.0], [3.0]]);
        let b = a.squeeze::<_, 1>();

        assert_eq!(b, Array1::from([1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_array2_matmul_identity() {
        let a = Array2::<f64, 2, 2>::from([[1.0, 0.0], [0.0, 1.0]]);
        let b = Array2::<f64, 2, 2>::from([[1.0, 2.0], [3.0, 4.0]]);

        assert_eq!(a.matmul(b), b);
    }

    #[test]
    fn test_array2_matmul_negative_y() {
        let a = Array2::<f64, 2, 2>::from([[1.0, 0.0], [0.0, -1.0]]);
        let b = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let c = a.matmul(b);

        assert_eq!(c, Array2::from([[1.0, 2.0, 3.0], [-4.0, -5.0, -6.0]]));
    }

    #[test]
    fn test_array2_matmul_negative_x() {
        let a = Array2::<f64, 2, 2>::from([[-1.0, 0.0], [0.0, 1.0]]);
        let b = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let c = a.matmul(b);

        assert_eq!(c, Array2::from([[-1.0, -2.0, -3.0], [4.0, 5.0, 6.0]]));
    }

    #[test]
    fn test_array2_matmul_array1_identity() {
        let a = Array2::<f64, 2, 2>::from([[1.0, 0.0], [0.0, 1.0]]);
        let b = Array1::<f64, 2>::from([1.0, 2.0]);
        let c = a.matmul(b);

        assert_eq!(c, Array1::from([1.0, 2.0]));
    }

    #[test]
    fn test_array2_matmul_array1_negative_y() {
        let a = Array2::<f64, 2, 2>::from([[1.0, 0.0], [0.0, -1.0]]);
        let b = Array1::<f64, 2>::from([1.0, 2.0]);
        let c = a.matmul(b);

        assert_eq!(c, Array1::from([1.0, -2.0]));
    }

    #[test]
    fn test_array2_matmul_array1_negative_x() {
        let a = Array2::<f64, 2, 2>::from([[-1.0, 0.0], [0.0, 1.0]]);
        let b = Array1::<f64, 2>::from([1.0, 2.0]);
        let c = a.matmul(b);

        assert_eq!(c, Array1::from([-1.0, 2.0]));
    }

    #[test]
    fn test_array2_reshape_array1() {
        let a = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b: Array1<f64, 6> = a.reshape();

        assert_eq!(b, Array1::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
    }

    #[test]
    fn test_array2_reshape_array2() {
        let a = Array2::<f64, 2, 3>::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let b: Array2<f64, 3, 2> = a.reshape();

        assert_eq!(b, Array2::from([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]));
    }
}
