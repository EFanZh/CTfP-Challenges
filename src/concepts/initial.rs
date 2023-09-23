use std::convert::Infallible;

pub trait Initial {
    fn into<T>(self) -> T;
}

impl Initial for Infallible {
    fn into<T>(self) -> T {
        match self {}
    }
}
