pub use permutations::*;
pub use rotations::*;
use std::ops::ControlFlow;

mod permutations;
mod rotations;

pub trait Fallible {
    type Output;
    type Residual;
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
    fn from_output(o: Self::Output) -> Self;
    fn from_residual(r: Self::Residual) -> Self;
}

impl<T, E> Fallible for Result<T, E> {
    type Output = T;
    type Residual = E;

    fn branch(self) -> ControlFlow<E, T> {
        match self {
            Ok(v) => ControlFlow::Continue(v),
            Err(e) => ControlFlow::Break(e),
        }
    }
    fn from_output(o: Self::Output) -> Self {
        Ok(o)
    }

    fn from_residual(r: Self::Residual) -> Self {
        Err(r)
    }
}

impl<T> Fallible for Option<T> {
    type Output = T;
    type Residual = ();

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Some(o) => ControlFlow::Continue(o),
            None => ControlFlow::Break(()),
        }
    }

    fn from_output(o: Self::Output) -> Self {
        Some(o)
    }

    fn from_residual(_r: Self::Residual) -> Self {
        None
    }
}
