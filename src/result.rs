use std::fmt::{Debug, Display, Formatter};

pub type Result<T> = std::result::Result<T, MagickError>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct MagickError(pub &'static str);

impl From<&'static str> for MagickError {
    fn from(s: &'static str) -> Self {
        MagickError(s)
    }
}

impl Display for MagickError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.0, f)
    }
}

impl std::error::Error for MagickError {}
