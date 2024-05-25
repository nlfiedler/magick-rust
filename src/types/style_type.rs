use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StyleType {
    Undefined = bindings::StyleType_UndefinedStyle,
    Normal = bindings::StyleType_NormalStyle,
    Italic = bindings::StyleType_ItalicStyle,
    Oblique = bindings::StyleType_ObliqueStyle,
    Any = bindings::StyleType_AnyStyle,
    Bold = bindings::StyleType_BoldStyle,
}

impl Default for StyleType {
    fn default() -> Self {
        return StyleType::Undefined;
    }
}

impl From<StyleType> for bindings::StyleType {
    fn from(value: StyleType) -> Self {
        return value as bindings::StyleType;
    }
}

impl From<bindings::StyleType> for StyleType {
    fn from(value: bindings::StyleType) -> Self {
        /*
         * SAFETY:
         *
         * `StyleType` has the same repr as `bindings::StyleType` - u32
         *
         * If `value` is less than Bold than it is in the vaild range and can be safely
         * reinterpreted as `StyleType`
         */
        if value <= bindings::StyleType_BoldStyle {
            return unsafe { std::mem::transmute(value) };
        }
        return StyleType::default();
    }
}
