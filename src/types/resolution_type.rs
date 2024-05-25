use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ResolutionType {
    Undefined = bindings::ResolutionType_UndefinedResolution,
    PixelsPerInch = bindings::ResolutionType_PixelsPerInchResolution,
    PixelsPerCentimeter = bindings::ResolutionType_PixelsPerCentimeterResolution,
}

impl Default for ResolutionType {
    fn default() -> Self {
        return ResolutionType::Undefined;
    }
}

impl From<ResolutionType> for bindings::ResolutionType {
    fn from(value: ResolutionType) -> Self {
        return value as bindings::ResolutionType;
    }
}

impl From<bindings::ResolutionType> for ResolutionType {
    fn from(value: bindings::ResolutionType) -> Self {
        /*
         * SAFETY:
         *
         * `ResolutionType` has the same repr as `bindings::ResolutionType` - u32
         *
         * If `value` is less than PixelsPerCentimeter than it is in the vaild range and can be safely
         * reinterpreted as `ResolutionType`
         */
        if value <= bindings::ResolutionType_PixelsPerCentimeterResolution {
            return unsafe { std::mem::transmute(value) };
        }
        return ResolutionType::default();
    }
}
