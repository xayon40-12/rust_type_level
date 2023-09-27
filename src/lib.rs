use std::marker::PhantomData;

mod private {
    pub trait Sealed {}
}
use private::Sealed;

pub trait Num: Sealed {}

pub struct Z {}
impl Sealed for Z {}
impl Num for Z {}

pub struct I<N: Num = Z>(PhantomData<N>);
impl<N: Num> Sealed for I<N> {}
impl<N: Num> Num for I<N> {}

pub struct O<N: Num = Z>(PhantomData<N>);
impl<N: Num> Sealed for O<N> {}
impl<N: Num> Num for O<N> {}

trait ReifyNum: Num {
    fn reify(u: usize) -> usize;
}
impl ReifyNum for Z {
    fn reify(u: usize) -> usize {
        u
    }
}
impl<N: Num + ReifyNum> ReifyNum for I<N> {
    fn reify(u: usize) -> usize {
        N::reify(1 + 2 * u)
    }
}
impl<N: Num + ReifyNum> ReifyNum for O<N> {
    fn reify(u: usize) -> usize {
        N::reify(2 * u)
    }
}
pub trait Value<T> {
    fn value() -> T;
}
impl<N: ReifyNum> Value<usize> for N {
    fn value() -> usize {
        N::reify(0)
    }
}

pub trait RevNum<N: Num>: Num {
    type Output: Num;
}
impl<N: Num> RevNum<N> for Z {
    type Output = N;
}
impl<N: Num, M: Num + RevNum<O<N>>> RevNum<N> for O<M> {
    type Output = <M as RevNum<O<N>>>::Output;
}
impl<N: Num, M: Num + RevNum<I<N>>> RevNum<N> for I<M> {
    type Output = <M as RevNum<I<N>>>::Output;
}
pub type Rev<N> = <N as RevNum<Z>>::Output;

mod private_inc {
    use super::*;
    pub trait IncNumRev: Num {
        type Output: Num;
    }
}
use private_inc::IncNumRev;
impl IncNumRev for Z {
    type Output = I;
}
impl<N: Num> IncNumRev for O<N> {
    type Output = I<N>;
}
impl<N: Num + IncNumRev> IncNumRev for I<N> {
    type Output = O<IncRev<N>>;
}
type IncRev<N> = <N as IncNumRev>::Output;

pub trait IncNum: Num {
    type Output: Num;
}
impl<N: Num + RevNum<Z>> IncNum for N
where
    Rev<N>: IncNumRev,
    IncRev<Rev<N>>: RevNum<Z>,
{
    type Output = Rev<IncRev<Rev<N>>>;
}
pub type Inc<N> = <N as IncNum>::Output;

mod private_add {
    use super::*;
    pub trait AddNumRev<N: Num>: Num {
        type Output: Num;
    }
}
use private_add::AddNumRev;
impl<N: Num + Sealed> AddNumRev<Z> for N {
    type Output = N;
}
impl<N: Num + AddNumRev<M>, M: Num> AddNumRev<O<M>> for O<N> {
    type Output = O<AddRev<N, M>>;
}
impl<N: Num + AddNumRev<M>, M: Num> AddNumRev<I<M>> for O<N> {
    type Output = I<AddRev<N, M>>;
}
impl<N: Num + AddNumRev<M>, M: Num> AddNumRev<O<M>> for I<N> {
    type Output = I<AddRev<N, M>>;
}
impl<N: Num + AddNumRev<M> + IncNumRev, M: Num> AddNumRev<I<M>> for I<N>
where
    <N as IncNumRev>::Output: AddNumRev<M>,
{
    type Output = O<AddRev<IncRev<N>, M>>;
}
type AddRev<N, M> = <N as AddNumRev<M>>::Output;

pub trait AddNum<N: Num>: Num {
    type Output: Num;
}
impl<N: Num + RevNum<Z>, M: Num + RevNum<Z>> AddNum<N> for M
where
    Rev<N>: AddNumRev<Rev<M>>,
    AddRev<Rev<N>, Rev<M>>: RevNum<Z>,
{
    type Output = Rev<AddRev<Rev<N>, Rev<M>>>;
}
pub type Add<N, M> = <N as AddNum<M>>::Output;
