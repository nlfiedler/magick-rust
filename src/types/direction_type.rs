use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DirectionType {
    Undefined = bindings::DirectionType_UndefinedDirection,
    RightToLeft = bindings::DirectionType_RightToLeftDirection,
    LeftToRight = bindings::DirectionType_LeftToRightDirection,
    TopToBottom = bindings::DirectionType_TopToBottomDirection,
}

impl Default for DirectionType {
    fn default() -> Self {
        return DirectionType::Undefined;
    }
}

impl From<DirectionType> for bindings::DirectionType {
    fn from(value: DirectionType) -> Self {
        return value as bindings::DirectionType;
    }
}

impl From<bindings::DirectionType> for DirectionType {
    fn from(value: bindings::DirectionType) -> Self {
        /*
         * SAFETY:
         *
         * `DirectionType` has the same repr as `bindings::DirectionType` - u32
         *
         * If `value` is less than TopToBottom than it is in the vaild range and can be safely
         * reinterpreted as `DirectionType`
         */
        if value <= bindings::DirectionType_TopToBottomDirection {
            return unsafe { std::mem::transmute(value) };
        }
        return DirectionType::default();
    }
}
