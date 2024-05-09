use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DitherMethod {
    Undefined      = bindings::DitherMethod_UndefinedDitherMethod,
    No             = bindings::DitherMethod_NoDitherMethod,
    Riemersma      = bindings::DitherMethod_RiemersmaDitherMethod,
    FloydSteinberg = bindings::DitherMethod_FloydSteinbergDitherMethod,
}

impl Default for DitherMethod {
    fn default() -> Self {
        return DitherMethod::No;
    }
}

impl From<DitherMethod> for bindings::DitherMethod {
    fn from(value: DitherMethod) -> Self {
        return value as bindings::DitherMethod;
    }
}
