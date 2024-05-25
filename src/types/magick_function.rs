use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MagickFunction {
    Undefined = bindings::MagickFunction_UndefinedFunction,
    Arcsin = bindings::MagickFunction_ArcsinFunction,
    Arctan = bindings::MagickFunction_ArctanFunction,
    Polynomial = bindings::MagickFunction_PolynomialFunction,
    Sinusoid = bindings::MagickFunction_SinusoidFunction,
}

impl Default for MagickFunction {
    fn default() -> Self {
        return MagickFunction::Undefined;
    }
}

impl From<MagickFunction> for bindings::MagickFunction {
    fn from(value: MagickFunction) -> Self {
        return value as bindings::MagickFunction;
    }
}
