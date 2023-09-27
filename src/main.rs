use rust_type_level::*;

fn main() {
    type A = I<I<O>>;
    type B = I<O<I>>;
    let a: usize = A::value();
    let b: usize = B::value();
    let reva: usize = <Rev<A>>::value();
    let apb: usize = <Add<A, B>>::value();
    let ap1: usize = <Inc<A>>::value();
    let ap2: usize = <Inc<Inc<A>>>::value();
    println!("a: {a}\nb: {b}\nrev a: {reva}\na+b: {apb}");
    println!("a+1: {ap1}\na+2: {ap2}");
}
