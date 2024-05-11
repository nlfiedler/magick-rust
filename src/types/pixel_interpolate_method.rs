use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PixelInterpolateMethod {
    Undefined  = bindings::PixelInterpolateMethod_UndefinedInterpolatePixel,
    Average    = bindings::PixelInterpolateMethod_AverageInterpolatePixel,
    Average9   = bindings::PixelInterpolateMethod_Average9InterpolatePixel,
    Average16  = bindings::PixelInterpolateMethod_Average16InterpolatePixel,
    Background = bindings::PixelInterpolateMethod_BackgroundInterpolatePixel,
    Bilinear   = bindings::PixelInterpolateMethod_BilinearInterpolatePixel,
    Blend      = bindings::PixelInterpolateMethod_BlendInterpolatePixel,
    Catrom     = bindings::PixelInterpolateMethod_CatromInterpolatePixel,
    Integer    = bindings::PixelInterpolateMethod_IntegerInterpolatePixel,
    Mesh       = bindings::PixelInterpolateMethod_MeshInterpolatePixel,
    Nearest    = bindings::PixelInterpolateMethod_NearestInterpolatePixel,
    Spline     = bindings::PixelInterpolateMethod_SplineInterpolatePixel,
}

impl Default for PixelInterpolateMethod {
    fn default() -> Self {
        return PixelInterpolateMethod::Undefined;
    }
}

impl From<PixelInterpolateMethod> for bindings::PixelInterpolateMethod {
    fn from(value: PixelInterpolateMethod) -> Self {
        return value as bindings::PixelInterpolateMethod;
    }
}

impl From<bindings::PixelInterpolateMethod> for PixelInterpolateMethod {
    fn from(value: bindings::PixelInterpolateMethod) -> Self {
        /*
         * SAFETY:
         *
         * `PixelInterpolateMethod` has the same repr as `bindings::PixelInterpolateMethod` - u32
         *
         * If `value` is less than Spline than it is in the vaild range and can be safely
         * reinterpreted as `PixelInterpolateMethod`
         */
        if value <= bindings::PixelInterpolateMethod_SplineInterpolatePixel {
            return unsafe { std::mem::transmute(value) };
        }
        return PixelInterpolateMethod::default();
    }
}
