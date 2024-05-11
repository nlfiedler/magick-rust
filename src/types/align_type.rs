use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AlignType {
    Undefined = bindings::AlignType_UndefinedAlign,
    Left      = bindings::AlignType_LeftAlign,
    Center    = bindings::AlignType_CenterAlign,
    Right     = bindings::AlignType_RightAlign,
}

impl Default for AlignType {
    fn default() -> Self {
        return AlignType::Undefined;
    }
}

impl From<AlignType> for bindings::AlignType {
    fn from(value: AlignType) -> Self {
        return value as bindings::AlignType;
    }
}

impl From<bindings::AlignType> for AlignType {
    fn from(value: bindings::AlignType) -> Self {
        /*
         * SAFETY:
         *
         * `AlignType` has the same repr as `bindings::AlignType` - u32
         *
         * If `value` is less than Right than it is in the vaild range and can be safely
         * reinterpreted as `AlignType`
         */
        if value <= bindings::AlignType_RightAlign {
            return unsafe { std::mem::transmute(value) };
        }
        return AlignType::default();
    }
}
