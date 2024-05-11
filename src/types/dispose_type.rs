use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DisposeType {
    /*
     * Identical to `Undefined`
     */
    // Unrecognized = bindings::DisposeType_UnrecognizedDispose,
    Undefined    = bindings::DisposeType_UndefinedDispose,
    None         = bindings::DisposeType_NoneDispose,
    Background   = bindings::DisposeType_BackgroundDispose,
    Previous     = bindings::DisposeType_PreviousDispose,
}

impl Default for DisposeType {
    fn default() -> Self {
        return DisposeType::Undefined;
    }
}

impl From<DisposeType> for bindings::DisposeType {
    fn from(value: DisposeType) -> Self {
        return value as bindings::DisposeType;
    }
}

impl From<bindings::DisposeType> for DisposeType {
    fn from(value: bindings::DisposeType) -> Self {
        /*
         * SAFETY:
         *
         * `DisposeType` has the same repr as `bindings::DisposeType` - u32
         *
         * If `value` is less than Previous than it is in the vaild range and can be safely
         * reinterpreted as `DisposeType`
         */
        if value <= bindings::DisposeType_PreviousDispose {
            return unsafe { std::mem::transmute(value) };
        }
        return DisposeType::default();
    }
}
