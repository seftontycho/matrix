#![feature(generic_const_exprs)]

mod array1;
mod array2;
mod array3;
mod assert;
mod broadcast;
mod ops;

pub mod prelude {
    pub use crate::array1::*;
    pub use crate::array2::*;
    pub use crate::broadcast::*;
    pub use crate::ops::{Concat, MatMul, Squeeze, UnSqueeze};
}

// TODO:
//     Indexing
//     Broadcasting
//     Operations on referenced arrays (IE let c = &a + &b)
//     Array3, Array4, etc
//     More diverse constructors (IE from_shape, from_vec, linspace, arange, etc)

// Broadcasting:
//     - https://numpy.org/doc/stable/user/basics.broadcasting.html
//
//     trait Broadcast<Rhs> {
//         fn broadcast(self, rhs: Rhs) -> impl Iterator<Item = (T, U)>; // T and U are the types of the elements in the arrays
//     }
//
//     Then we can implement this trait for all combinations of Array1, Array2, Array3, etc where the shapes are compatible
//     Then we can implement various operations for pairs of types which implement Broadcast (IE Add, Sub, Mul, Div, etc)
