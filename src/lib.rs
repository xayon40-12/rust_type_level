use std::marker::PhantomData;

mod private {
    pub trait Sealed {}
}
use private::Sealed;

pub trait Num: Sealed {}

pub struct Z {}
impl Sealed for Z {}
impl Num for Z {}

pub struct I<N: Num = Z>(Box<N>, PhantomData<N>);
impl<N: Num> Sealed for I<N> {}
impl<N: Num> Num for I<N> {}

pub struct O<N: Num = Z>(Box<N>, PhantomData<N>);
impl<N: Num> Sealed for O<N> {}
impl<N: Num> Num for O<N> {}

pub trait SNum<N: Num>: Sealed {}

pub struct SZ {}
impl Sealed for SZ {}
impl SNum<Z> for SZ {}

pub struct SI<'a, N: Num + 'a = Z>(Box<dyn SNum<N> + 'a>);
impl<'a, N: Num + 'a> Sealed for SI<'a, N> {}
impl<'a, N: Num + 'a> SNum<I<N>> for SI<'a, N> {}

pub struct SO<'a, N: Num + 'a = Z>(Box<dyn SNum<N> + 'a>);
impl<'a, N: Num + 'a> Sealed for SO<'a, N> {}
impl<'a, N: Num + 'a> SNum<O<N>> for SO<'a, N> {}

pub trait SNumFromNum<N: Num, SN: SNum<N>> {
    fn fromnum(n: N) -> SN;
}
impl SNumFromNum<Z, SZ> for SZ {
    fn fromnum(_: Z) -> SZ {
        SZ {}
    }
}
impl<'a> SNumFromNum<I<Z>, SI<'a, Z>> for SI<'a, Z> {
    fn fromnum(I(n, _): I<Z>) -> SI<'a, Z> {
        SI(Box::new(<SZ>::fromnum(*n)))
    }
}
impl<'a> SNumFromNum<O<Z>, SO<'a, Z>> for SO<'a, Z> {
    fn fromnum(O(n, _): O<Z>) -> SO<'a, Z> {
        SO(Box::new(<SZ>::fromnum(*n)))
    }
}
impl<'a, N: Num + 'a> SNumFromNum<I<I<N>>, SI<'a, I<N>>> for SI<'a, I<N>>
where
    SI<'a, N>: SNumFromNum<I<N>, SI<'a, N>>,
{
    fn fromnum(I(n, _): I<I<N>>) -> SI<'a, I<N>> {
        SI(Box::new(<SI<N>>::fromnum(*n)))
    }
}
impl<'a, N: Num + 'a> SNumFromNum<I<O<N>>, SI<'a, O<N>>> for SI<'a, O<N>>
where
    SO<'a, N>: SNumFromNum<O<N>, SO<'a, N>>,
{
    fn fromnum(I(n, _): I<O<N>>) -> SI<'a, O<N>> {
        SI(Box::new(<SO<N>>::fromnum(*n)))
    }
}
impl<'a, N: Num + 'a> SNumFromNum<O<I<N>>, SO<'a, I<N>>> for SO<'a, I<N>>
where
    SI<'a, N>: SNumFromNum<I<N>, SI<'a, N>>,
{
    fn fromnum(O(n, _): O<I<N>>) -> SO<'a, I<N>> {
        SO(Box::new(<SI<N>>::fromnum(*n)))
    }
}
impl<'a, N: Num + 'a> SNumFromNum<O<O<N>>, SO<'a, O<N>>> for SO<'a, O<N>>
where
    SO<'a, N>: SNumFromNum<O<N>, SO<'a, N>>,
{
    fn fromnum(O(n, _): O<O<N>>) -> SO<'a, O<N>> {
        SO(Box::new(<SO<N>>::fromnum(*n)))
    }
}

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
