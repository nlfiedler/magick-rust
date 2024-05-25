use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum EndianType {
    Undefined = bindings::EndianType_UndefinedEndian,
    LSB = bindings::EndianType_LSBEndian,
    MSB = bindings::EndianType_MSBEndian,
}

impl Default for EndianType {
    fn default() -> Self {
        return EndianType::Undefined;
    }
}

impl From<EndianType> for bindings::EndianType {
    fn from(value: EndianType) -> Self {
        return value as bindings::EndianType;
    }
}

impl From<bindings::EndianType> for EndianType {
    fn from(value: bindings::EndianType) -> Self {
        /*
         * SAFETY:
         *
         * `EndianType` has the same repr as `bindings::EndianType` - u32
         *
         * If `value` is less than MSB than it is in the vaild range and can be safely
         * reinterpreted as `EndianType`
         */
        if value <= bindings::EndianType_MSBEndian {
            return unsafe { std::mem::transmute(value) };
        }
        return EndianType::default();
    }
}
