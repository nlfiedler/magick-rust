use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ClipPathUnits {
    Undefined = bindings::ClipPathUnits_UndefinedPathUnits,
    UserSpace = bindings::ClipPathUnits_UserSpace,
    UserSpaceOnUse = bindings::ClipPathUnits_UserSpaceOnUse,
    ObjectBoundingBox = bindings::ClipPathUnits_ObjectBoundingBox,
}

impl Default for ClipPathUnits {
    fn default() -> Self {
        return ClipPathUnits::Undefined;
    }
}

impl From<ClipPathUnits> for bindings::ClipPathUnits {
    fn from(value: ClipPathUnits) -> Self {
        return value as bindings::ClipPathUnits;
    }
}

impl From<bindings::ClipPathUnits> for ClipPathUnits {
    fn from(value: bindings::ClipPathUnits) -> Self {
        /*
         * SAFETY:
         *
         * `ClipPathUnits` has the same repr as `bindings::ClipPathUnits` - u32
         *
         * If `value` is less than ObjectBoundingBox than it is in the vaild range and can be safely
         * reinterpreted as `ClipPathUnits`
         */
        if value <= bindings::ClipPathUnits_ObjectBoundingBox {
            return unsafe { std::mem::transmute(value) };
        }
        return ClipPathUnits::default();
    }
}
