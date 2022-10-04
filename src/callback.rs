use serde::{de::DeserializeOwned, Deserialize};

pub trait On<'a> {
    type Arg: Deserialize<'a>;
}

pub trait OnAny: for<'any> On<'any> {}
impl<T: ?Sized + for<'any> On<'any>> OnAny for T {}

impl<T: DeserializeOwned> On<'_> for T {
    type Arg = T;
}

pub trait CallbackOn<'a, T: On<'a>>: FnMut(T::Arg) {}
impl<'a, T: On<'a>, F: FnMut(T::Arg)> CallbackOn<'a, T> for F {}

pub trait Callback<T: OnAny>: for<'any> CallbackOn<'any, T> {}
impl<T: OnAny, F: for<'any> CallbackOn<'any, T>> Callback<T> for F {}
