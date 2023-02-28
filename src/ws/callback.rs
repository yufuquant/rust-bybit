use serde::Deserialize;

pub trait Arg {
    type ValueType<'a>: Deserialize<'a>
    where
        Self: 'a;
}

pub trait Callback<A: Arg>: for<'any> FnMut(A::ValueType<'any>) {}
impl<A: Arg, F: for<'any> FnMut(A::ValueType<'any>)> Callback<A> for F {}
