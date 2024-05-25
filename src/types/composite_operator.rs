use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CompositeOperator {
    Undefined = bindings::CompositeOperator_UndefinedCompositeOp,
    Alpha = bindings::CompositeOperator_AlphaCompositeOp,
    Atop = bindings::CompositeOperator_AtopCompositeOp,
    Blend = bindings::CompositeOperator_BlendCompositeOp,
    Blur = bindings::CompositeOperator_BlurCompositeOp,
    Bumpmap = bindings::CompositeOperator_BumpmapCompositeOp,
    ChangeMask = bindings::CompositeOperator_ChangeMaskCompositeOp,
    Clear = bindings::CompositeOperator_ClearCompositeOp,
    ColorBurn = bindings::CompositeOperator_ColorBurnCompositeOp,
    ColorDodge = bindings::CompositeOperator_ColorDodgeCompositeOp,
    Colorize = bindings::CompositeOperator_ColorizeCompositeOp,
    CopyBlack = bindings::CompositeOperator_CopyBlackCompositeOp,
    CopyBlue = bindings::CompositeOperator_CopyBlueCompositeOp,
    Copy = bindings::CompositeOperator_CopyCompositeOp,
    CopyCyan = bindings::CompositeOperator_CopyCyanCompositeOp,
    CopyGreen = bindings::CompositeOperator_CopyGreenCompositeOp,
    CopyMagenta = bindings::CompositeOperator_CopyMagentaCompositeOp,
    CopyAlpha = bindings::CompositeOperator_CopyAlphaCompositeOp,
    CopyRed = bindings::CompositeOperator_CopyRedCompositeOp,
    CopyYellow = bindings::CompositeOperator_CopyYellowCompositeOp,
    Darken = bindings::CompositeOperator_DarkenCompositeOp,
    DarkenIntensity = bindings::CompositeOperator_DarkenIntensityCompositeOp,
    Difference = bindings::CompositeOperator_DifferenceCompositeOp,
    Displace = bindings::CompositeOperator_DisplaceCompositeOp,
    Dissolve = bindings::CompositeOperator_DissolveCompositeOp,
    Distort = bindings::CompositeOperator_DistortCompositeOp,
    DivideDst = bindings::CompositeOperator_DivideDstCompositeOp,
    DivideSrc = bindings::CompositeOperator_DivideSrcCompositeOp,
    DstAtop = bindings::CompositeOperator_DstAtopCompositeOp,
    Dst = bindings::CompositeOperator_DstCompositeOp,
    DstIn = bindings::CompositeOperator_DstInCompositeOp,
    DstOut = bindings::CompositeOperator_DstOutCompositeOp,
    DstOver = bindings::CompositeOperator_DstOverCompositeOp,
    Exclusion = bindings::CompositeOperator_ExclusionCompositeOp,
    HardLight = bindings::CompositeOperator_HardLightCompositeOp,
    HardMix = bindings::CompositeOperator_HardMixCompositeOp,
    Hue = bindings::CompositeOperator_HueCompositeOp,
    In = bindings::CompositeOperator_InCompositeOp,
    Intensity = bindings::CompositeOperator_IntensityCompositeOp,
    Lighten = bindings::CompositeOperator_LightenCompositeOp,
    LightenIntensity = bindings::CompositeOperator_LightenIntensityCompositeOp,
    LinearBurn = bindings::CompositeOperator_LinearBurnCompositeOp,
    LinearDodge = bindings::CompositeOperator_LinearDodgeCompositeOp,
    LinearLight = bindings::CompositeOperator_LinearLightCompositeOp,
    Luminize = bindings::CompositeOperator_LuminizeCompositeOp,
    Mathematics = bindings::CompositeOperator_MathematicsCompositeOp,
    MinusDst = bindings::CompositeOperator_MinusDstCompositeOp,
    MinusSrc = bindings::CompositeOperator_MinusSrcCompositeOp,
    Modulate = bindings::CompositeOperator_ModulateCompositeOp,
    ModulusAdd = bindings::CompositeOperator_ModulusAddCompositeOp,
    ModulusSubtract = bindings::CompositeOperator_ModulusSubtractCompositeOp,
    Multiply = bindings::CompositeOperator_MultiplyCompositeOp,
    No = bindings::CompositeOperator_NoCompositeOp,
    Out = bindings::CompositeOperator_OutCompositeOp,
    Over = bindings::CompositeOperator_OverCompositeOp,
    Overlay = bindings::CompositeOperator_OverlayCompositeOp,
    PegtopLight = bindings::CompositeOperator_PegtopLightCompositeOp,
    PinLight = bindings::CompositeOperator_PinLightCompositeOp,
    Plus = bindings::CompositeOperator_PlusCompositeOp,
    Replace = bindings::CompositeOperator_ReplaceCompositeOp,
    Saturate = bindings::CompositeOperator_SaturateCompositeOp,
    Screen = bindings::CompositeOperator_ScreenCompositeOp,
    SoftLight = bindings::CompositeOperator_SoftLightCompositeOp,
    SrcAtop = bindings::CompositeOperator_SrcAtopCompositeOp,
    Src = bindings::CompositeOperator_SrcCompositeOp,
    SrcIn = bindings::CompositeOperator_SrcInCompositeOp,
    SrcOut = bindings::CompositeOperator_SrcOutCompositeOp,
    SrcOver = bindings::CompositeOperator_SrcOverCompositeOp,
    Threshold = bindings::CompositeOperator_ThresholdCompositeOp,
    VividLight = bindings::CompositeOperator_VividLightCompositeOp,
    Xor = bindings::CompositeOperator_XorCompositeOp,
    Stereo = bindings::CompositeOperator_StereoCompositeOp,
    Freeze = bindings::CompositeOperator_FreezeCompositeOp,
    Interpolate = bindings::CompositeOperator_InterpolateCompositeOp,
    Negate = bindings::CompositeOperator_NegateCompositeOp,
    Reflect = bindings::CompositeOperator_ReflectCompositeOp,
    SoftBurn = bindings::CompositeOperator_SoftBurnCompositeOp,
    SoftDodge = bindings::CompositeOperator_SoftDodgeCompositeOp,
    Stamp = bindings::CompositeOperator_StampCompositeOp,
    RMSE = bindings::CompositeOperator_RMSECompositeOp,
    SaliencyBlend = bindings::CompositeOperator_SaliencyBlendCompositeOp,
    SeamlessBlend = bindings::CompositeOperator_SeamlessBlendCompositeOp,
}

impl Default for CompositeOperator {
    fn default() -> Self {
        return CompositeOperator::Over;
    }
}

impl From<CompositeOperator> for bindings::CompositeOperator {
    fn from(value: CompositeOperator) -> Self {
        return value as bindings::CompositeOperator;
    }
}

impl From<bindings::CompositeOperator> for CompositeOperator {
    fn from(value: bindings::CompositeOperator) -> Self {
        /*
         * SAFETY:
         *
         * `CompositeOperator` has the same repr as `bindings::CompositeOperator` - u32
         *
         * If `value` is less than SeamlessBlend than it is in the vaild range and can be safely
         * reinterpreted as `CompositeOperator`
         */
        if value <= bindings::CompositeOperator_SeamlessBlendCompositeOp {
            return unsafe { std::mem::transmute(value) };
        }
        return CompositeOperator::default();
    }
}
