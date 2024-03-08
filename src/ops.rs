pub trait Concat<Rhs, Out> {
    fn concat(self, other: Rhs) -> Out;
}

pub trait UnSqueezeInner<Out, const AXIS: usize> {
    fn unsqueeze_(self) -> Out;
}

pub trait UnSqueeze: Sized {
    fn unsqueeze<U, const AXIS: usize>(self) -> U
    where
        Self: UnSqueezeInner<U, AXIS>,
    {
        self.unsqueeze_()
    }
}

impl<T> UnSqueeze for T {}

pub trait SqueezeInner<Out, const AXIS: usize> {
    fn squeeze_(self) -> Out;
}

pub trait Squeeze: Sized {
    fn squeeze<U, const AXIS: usize>(self) -> U
    where
        Self: SqueezeInner<U, AXIS>,
    {
        self.squeeze_()
    }
}

impl<T> Squeeze for T {}

pub trait MatMul<Rhs, Out> {
    fn matmul(self, rhs: Rhs) -> Out;
}

pub trait ReshapeInner<Out> {
    fn reshape_(self) -> Out;
}

pub trait Reshape: Sized {
    fn reshape<U>(self) -> U
    where
        Self: ReshapeInner<U>,
    {
        self.reshape_()
    }
}

impl<T> Reshape for T {}
