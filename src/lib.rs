#![feature(generic_const_exprs)]

mod array1;
mod array2;
mod array3;
mod assert;
mod ops;

pub mod prelude {
    pub use crate::array1::Array1;
    pub use crate::array2::Array2;
    pub use crate::ops::{Concat, MatMul, Squeeze, UnSqueeze};
}

// TODO:
//     Indexing
//     Broadcasting
//     Operations on referenced arrays (IE let c = &a + &b)
//     Array3, Array4, etc
//     More diverse constructors (IE from_shape, from_vec, linspace, arange, etc)
