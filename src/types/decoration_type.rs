use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DecorationType {
    Undefined   = bindings::DecorationType_UndefinedDecoration,
    No          = bindings::DecorationType_NoDecoration,
    Underline   = bindings::DecorationType_UnderlineDecoration,
    Overline    = bindings::DecorationType_OverlineDecoration,
    LineThrough = bindings::DecorationType_LineThroughDecoration,
}

impl Default for DecorationType {
    fn default() -> Self {
        return DecorationType::Undefined;
    }
}

impl From<DecorationType> for bindings::DecorationType {
    fn from(value: DecorationType) -> Self {
        return value as bindings::DecorationType;
    }
}

impl From<bindings::DecorationType> for DecorationType {
    fn from(value: bindings::DecorationType) -> Self {
        /*
         * SAFETY:
         *
         * `DecorationType` has the same repr as `bindings::DecorationType` - u32
         *
         * If `value` is less than LineThrough than it is in the vaild range and can be safely
         * reinterpreted as `DecorationType`
         */
        if value <= bindings::DecorationType_LineThroughDecoration {
            return unsafe { std::mem::transmute(value) };
        }
        return DecorationType::default();
    }
}
