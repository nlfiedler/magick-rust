use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum OrientationType {
    Undefined   = bindings::OrientationType_UndefinedOrientation,
    TopLeft     = bindings::OrientationType_TopLeftOrientation,
    TopRight    = bindings::OrientationType_TopRightOrientation,
    BottomRight = bindings::OrientationType_BottomRightOrientation,
    BottomLeft  = bindings::OrientationType_BottomLeftOrientation,
    LeftTop     = bindings::OrientationType_LeftTopOrientation,
    RightTop    = bindings::OrientationType_RightTopOrientation,
    RightBottom = bindings::OrientationType_RightBottomOrientation,
    LeftBottom  = bindings::OrientationType_LeftBottomOrientation,
}

impl Default for OrientationType {
    fn default() -> Self {
        return OrientationType::Undefined;
    }
}

impl From<OrientationType> for bindings::OrientationType {
    fn from(value: OrientationType) -> Self {
        return value as bindings::OrientationType;
    }
}

impl From<bindings::OrientationType> for OrientationType {
    fn from(value: bindings::OrientationType) -> Self {
        /*
         * SAFETY:
         *
         * `OrientationType` has the same repr as `bindings::OrientationType` - u32
         *
         * If `value` is less than LeftBottom than it is in the vaild range and can be safely
         * reinterpreted as `OrientationType`
         */
        if value <= bindings::OrientationType_LeftBottomOrientation {
            return unsafe { std::mem::transmute(value) };
        }
        return OrientationType::default();
    }
}
