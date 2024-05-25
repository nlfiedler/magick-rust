use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AutoThresholdMethod {
    Undefined = bindings::AutoThresholdMethod_UndefinedThresholdMethod,
    Kapur = bindings::AutoThresholdMethod_KapurThresholdMethod,
    OTSU = bindings::AutoThresholdMethod_OTSUThresholdMethod,
    Triangle = bindings::AutoThresholdMethod_TriangleThresholdMethod,
}

impl Default for AutoThresholdMethod {
    fn default() -> Self {
        return AutoThresholdMethod::Undefined;
    }
}

impl From<AutoThresholdMethod> for bindings::AutoThresholdMethod {
    fn from(value: AutoThresholdMethod) -> Self {
        return value as bindings::AutoThresholdMethod;
    }
}
