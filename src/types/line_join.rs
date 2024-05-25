use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LineJoin {
    Undefined = bindings::LineJoin_UndefinedJoin,
    Miter = bindings::LineJoin_MiterJoin,
    Round = bindings::LineJoin_RoundJoin,
    Bevel = bindings::LineJoin_BevelJoin,
}

impl Default for LineJoin {
    fn default() -> Self {
        return LineJoin::Undefined;
    }
}

impl From<LineJoin> for bindings::LineJoin {
    fn from(value: LineJoin) -> Self {
        return value as bindings::LineJoin;
    }
}

impl From<bindings::LineJoin> for LineJoin {
    fn from(value: bindings::LineJoin) -> Self {
        /*
         * SAFETY:
         *
         * `LineJoin` has the same repr as `bindings::LineJoin` - u32
         *
         * If `value` is less than Bevel than it is in the vaild range and can be safely
         * reinterpreted as `LineJoin`
         */
        if value <= bindings::LineJoin_BevelJoin {
            return unsafe { std::mem::transmute(value) };
        }
        return LineJoin::default();
    }
}
