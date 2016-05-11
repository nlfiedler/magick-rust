use super::bindings;

pub trait FromRust<T> {
    fn from_rust(t: T) -> Self;
}

impl FromRust<bool> for bindings::MagickBooleanType {
    fn from_rust(b: bool) -> Self {
        if b {
            bindings::MagickTrue
        } else {
            bindings::MagickFalse
        }
    }
}

pub trait ToMagick<T> {
    fn to_magick(self) -> T;
}

impl<T, E> ToMagick<T> for E
    where T: FromRust<E>
{
    fn to_magick(self) -> T {
        <T as FromRust<E>>::from_rust(self)
    }
}
