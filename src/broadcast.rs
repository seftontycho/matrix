use crate::array1::Array1;
use crate::array2::Array2;
use crate::array3::Array3;
use crate::assert::{Assert, True};

const fn dimensions_compatible(a: usize, b: usize) -> bool {
    a == b || a == 1 || b == 1
}

pub const fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

#[macro_export]
macro_rules! resulting_shape {
    ( $( $a:expr),+ , $( $b:expr),+ ) => {
        [$( max($a, $b) ),+]
    };
}

/// When operating on two arrays, NumPy compares their shapes element-wise.
/// It starts with the trailing (i.e. rightmost) dimension and works its way left.
///
/// Two dimensions are compatible when:
/// - they are equal, or
/// - one of them is 1.
pub trait Broadcast<Rhs, T, U, V> {
    type Output;

    fn broadcast(self, rhs: Rhs, op: impl Fn(&T, &U) -> V) -> Self::Output;
}

impl<T, U, V, const A: usize> Broadcast<U, T, U, V> for Array1<T, A>
where
    Assert<{ dimensions_compatible(A, 1) }>: True,
{
    type Output = Array1<V, A>;

    fn broadcast(self, rhs: U, op: impl Fn(&T, &U) -> V) -> Self::Output {
        let result: [_; A] = std::array::from_fn(|i| op(&self.data[i], &rhs));

        result.into()
    }
}

impl<T, U, V, const A: usize, const B: usize> Broadcast<Array1<U, B>, T, U, V> for Array1<T, A>
where
    Assert<{ dimensions_compatible(A, B) }>: True,
    [(); max(A, B)]:,
{
    type Output = Array1<V, { max(A, B) }>;

    fn broadcast(self, rhs: Array1<U, B>, op: impl Fn(&T, &U) -> V) -> Self::Output {
        let result: [_; max(A, B)] = std::array::from_fn(|i| {
            let a = if A == 1 { &self.data[0] } else { &self.data[i] };
            let b = if B == 1 { &rhs.data[0] } else { &rhs.data[i] };

            op(a, b)
        });

        result.into()
    }
}

impl<T, U, V, const A: usize, const B: usize> Broadcast<U, T, U, V> for Array2<T, A, B>
where
    Assert<{ dimensions_compatible(B, 1) }>: True,
{
    type Output = Array2<V, A, B>;

    fn broadcast(self, rhs: U, op: impl Fn(&T, &U) -> V) -> Self::Output {
        let result: [[_; B]; A] =
            std::array::from_fn(|i| std::array::from_fn(|j| op(&self.data[i][j], &rhs)));

        result.into()
    }
}

impl<T, U, V, const A: usize, const B: usize, const C: usize> Broadcast<Array1<U, C>, T, U, V>
    for Array2<T, A, B>
where
    Assert<{ dimensions_compatible(B, C) }>: True,
    [(); max(B, C)]:,
{
    type Output = Array2<V, A, { max(B, C) }>;

    fn broadcast(self, rhs: Array1<U, C>, op: impl Fn(&T, &U) -> V) -> Self::Output {
        let result: [[_; max(B, C)]; A] = std::array::from_fn(|i| {
            let row = &self.data[i];

            std::array::from_fn(|j| {
                let b = if B == 1 { &row[0] } else { &row[j] };
                let c = if C == 1 { &rhs.data[0] } else { &rhs.data[j] };

                op(b, c)
            })
        });

        result.into()
    }
}

impl<T, U, V, const A: usize, const B: usize, const C: usize, const D: usize>
    Broadcast<Array2<U, C, D>, T, U, V> for Array2<T, A, B>
where
    Assert<{ dimensions_compatible(A, C) }>: True,
    Assert<{ dimensions_compatible(B, D) }>: True,
    [(); max(A, C)]:,
    [(); max(B, D)]:,
{
    type Output = Array2<V, { max(A, C) }, { max(B, D) }>;

    fn broadcast(self, rhs: Array2<U, C, D>, op: impl Fn(&T, &U) -> V) -> Self::Output {
        let result: [[_; max(B, D)]; max(A, C)] = std::array::from_fn(|i| {
            let row_lhs = if A == 1 { &self.data[0] } else { &self.data[i] };
            let row_rhs = if C == 1 { &rhs.data[0] } else { &rhs.data[i] };

            std::array::from_fn(|j| {
                let b = if B == 1 { &row_lhs[0] } else { &row_lhs[j] };
                let d = if D == 1 { &row_rhs[0] } else { &row_rhs[j] };

                op(b, d)
            })
        });

        result.into()
    }
}

impl<T, U, V, const A: usize, const B: usize, const C: usize> Broadcast<U, T, U, V>
    for Array3<T, A, B, C>
where
    Assert<{ dimensions_compatible(C, 1) }>: True,
{
    type Output = Array3<V, A, B, C>;

    fn broadcast(self, rhs: U, op: impl Fn(&T, &U) -> V) -> Self::Output {
        let result: [[[_; C]; B]; A] = std::array::from_fn(|i| {
            std::array::from_fn(|j| std::array::from_fn(|k| op(&self.data[i][j][k], &rhs)))
        });

        result.into()
    }
}

impl<T, U, V, const A: usize, const B: usize, const C: usize, const D: usize>
    Broadcast<Array1<U, D>, T, U, V> for Array3<T, A, B, C>
where
    Assert<{ dimensions_compatible(C, D) }>: True,
    [(); max(C, D)]:,
{
    type Output = Array3<V, A, B, { max(C, D) }>;

    fn broadcast(self, rhs: Array1<U, D>, op: impl Fn(&T, &U) -> V) -> Self::Output {
        let result: [[[_; max(C, D)]; B]; A] = std::array::from_fn(|i| {
            std::array::from_fn(|j| {
                let row = &self.data[i][j];

                std::array::from_fn(|k| {
                    let c = if C == 1 { &row[0] } else { &row[k] };
                    let d = if D == 1 { &rhs.data[0] } else { &rhs.data[k] };

                    op(c, d)
                })
            })
        });

        result.into()
    }
}

impl<T, U, V, const A: usize, const B: usize, const C: usize, const D: usize, const E: usize>
    Broadcast<Array2<U, D, E>, T, U, V> for Array3<T, A, B, C>
where
    Assert<{ dimensions_compatible(B, D) }>: True,
    Assert<{ dimensions_compatible(C, E) }>: True,
    [(); max(B, D)]:,
    [(); max(C, E)]:,
{
    type Output = Array3<V, A, { max(B, D) }, { max(C, E) }>;

    fn broadcast(self, rhs: Array2<U, D, E>, op: impl Fn(&T, &U) -> V) -> Self::Output {
        let result: [[[_; max(C, E)]; max(B, D)]; A] = std::array::from_fn(|i| {
            let floor_lhs = &self.data[i];

            std::array::from_fn(|j| {
                let row_lhs = if B == 1 { &floor_lhs[0] } else { &floor_lhs[j] };
                let row_rhs = if D == 1 { &rhs.data[0] } else { &rhs.data[j] };

                std::array::from_fn(|k| {
                    let c = if C == 1 { &row_lhs[0] } else { &row_lhs[k] };
                    let e = if E == 1 { &row_rhs[0] } else { &row_rhs[k] };

                    op(c, e)
                })
            })
        });

        result.into()
    }
}

impl<
        T,
        U,
        V,
        const A: usize,
        const B: usize,
        const C: usize,
        const D: usize,
        const E: usize,
        const F: usize,
    > Broadcast<Array3<U, D, E, F>, T, U, V> for Array3<T, A, B, C>
where
    Assert<{ dimensions_compatible(A, D) }>: True,
    Assert<{ dimensions_compatible(B, E) }>: True,
    Assert<{ dimensions_compatible(C, F) }>: True,
    [(); max(A, D)]:,
    [(); max(B, E)]:,
    [(); max(C, F)]:,
{
    type Output = Array3<V, { max(A, D) }, { max(B, E) }, { max(C, F) }>;

    fn broadcast(self, rhs: Array3<U, D, E, F>, op: impl Fn(&T, &U) -> V) -> Self::Output {
        let result: [[[_; max(C, F)]; max(B, E)]; max(A, D)] = std::array::from_fn(|i| {
            let floor_lhs = if A == 1 { &self.data[0] } else { &self.data[i] };
            let floor_rhs = if D == 1 { &rhs.data[0] } else { &rhs.data[i] };

            std::array::from_fn(|j| {
                let row_lhs = if B == 1 { &floor_lhs[0] } else { &floor_lhs[j] };
                let row_rhs = if E == 1 { &floor_rhs[0] } else { &floor_rhs[j] };

                std::array::from_fn(|k| {
                    let c = if C == 1 { &row_lhs[0] } else { &row_lhs[k] };
                    let f = if F == 1 { &row_rhs[0] } else { &row_rhs[k] };

                    op(c, f)
                })
            })
        });

        result.into()
    }
}
