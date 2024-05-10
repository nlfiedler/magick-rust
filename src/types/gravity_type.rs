use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum GravityType {
    Undefined = bindings::GravityType_UndefinedGravity,
    /*
     * Identical to `Undefined`
     */
    // Forget    = bindings::GravityType_ForgetGravity,
    NorthWest = bindings::GravityType_NorthWestGravity,
    North     = bindings::GravityType_NorthGravity,
    NorthEast = bindings::GravityType_NorthEastGravity,
    West      = bindings::GravityType_WestGravity,
    Center    = bindings::GravityType_CenterGravity,
    East      = bindings::GravityType_EastGravity,
    SouthWest = bindings::GravityType_SouthWestGravity,
    South     = bindings::GravityType_SouthGravity,
    SouthEast = bindings::GravityType_SouthEastGravity,
}

impl Default for GravityType {
    fn default() -> Self {
        return GravityType::Undefined;
    }
}

impl From<GravityType> for bindings::GravityType {
    fn from(value: GravityType) -> Self {
        return value as bindings::GravityType;
    }
}

impl From<bindings::GravityType> for GravityType {
    fn from(value: bindings::GravityType) -> Self {
        /*
         * SAFETY:
         *
         * `GravityType` has the same repr as `bindings::GravityType` - u32
         *
         * If `value` is less than SouthEast than it is in the vaild range and can be safely
         * reinterpreted as `GravityType`
         */
        if value <= bindings::GravityType_SouthEastGravity {
            return unsafe { std::mem::transmute(value) };
        }
        return GravityType::default();
    }
}
