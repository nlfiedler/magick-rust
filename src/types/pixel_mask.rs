use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PixelMask {
    Undefined = bindings::PixelMask_UndefinedPixelMask,
    Read      = bindings::PixelMask_ReadPixelMask,
    Write     = bindings::PixelMask_WritePixelMask,
    Composite = bindings::PixelMask_CompositePixelMask,
}

impl Default for PixelMask {
    fn default() -> Self {
        return PixelMask::Undefined;
    }
}

impl From<PixelMask> for bindings::PixelMask {
    fn from(value: PixelMask) -> Self {
        return value as bindings::PixelMask;
    }
}
