pub enum CompositeOperator {
    Undefined,
    Alpha,
    Atop,
    Blend,
    Blur,
    Bumpmap,
    ChangeMask,
    Clear,
    ColorBurn,
    ColorDodge,
    Colorize,
    CopyBlack,
    CopyBlue,
    Copy,
    CopyCyan,
    CopyGreen,
    CopyMagenta,
    CopyAlpha,
    CopyRed,
    CopyYellow,
    Darken,
    DarkenIntensity,
    Difference,
    Displace,
    Dissolve,
    Distort,
    DivideDst,
    DivideSrc,
    DstAtop,
    Dst,
    DstIn,
    DstOut,
    DstOver,
    Exclusion,
    HardLight,
    HardMix,
    Hue,
    In,
    Intensity,
    Lighten,
    LightenIntensity,
    LinearBurn,
    LinearDodge,
    LinearLight,
    Luminize,
    Mathematics,
    MinusDst,
    MinusSrc,
    Modulate,
    ModulusAdd,
    ModulusSubtract,
    Multiply,
    No,
    Out,
    Over,
    Overlay,
    PegtopLight,
    PinLight,
    Plus,
    Replace,
    Saturate,
    Screen,
    SoftLight,
    SrcAtop,
    Src,
    SrcIn,
    SrcOut,
    SrcOver,
    Threshold,
    VividLight,
    Xor,
    Stereo,
    Freeze,
    Interpolate,
    Negate,
    Reflect,
    SoftBurn,
    SoftDodge,
    Stamp,
    RMSE,
    SaliencyBlend,
    SeamlessBlend,
}

impl From<CompositeOperator> for crate::bindings::CompositeOperator {
    fn from(value: CompositeOperator) -> Self {
        match value {
            CompositeOperator::Undefined => crate::bindings::CompositeOperator_UndefinedCompositeOp,
            CompositeOperator::Alpha => crate::bindings::CompositeOperator_AlphaCompositeOp,
            CompositeOperator::Atop => crate::bindings::CompositeOperator_AtopCompositeOp,
            CompositeOperator::Blend => crate::bindings::CompositeOperator_BlendCompositeOp,
            CompositeOperator::Blur => crate::bindings::CompositeOperator_BlurCompositeOp,
            CompositeOperator::Bumpmap => crate::bindings::CompositeOperator_BumpmapCompositeOp,
            CompositeOperator::ChangeMask => {
                crate::bindings::CompositeOperator_ChangeMaskCompositeOp
            }
            CompositeOperator::Clear => crate::bindings::CompositeOperator_ClearCompositeOp,
            CompositeOperator::ColorBurn => crate::bindings::CompositeOperator_ColorBurnCompositeOp,
            CompositeOperator::ColorDodge => {
                crate::bindings::CompositeOperator_ColorDodgeCompositeOp
            }
            CompositeOperator::Colorize => crate::bindings::CompositeOperator_ColorizeCompositeOp,
            CompositeOperator::CopyBlack=> {
                crate::bindings::CompositeOperator_CopyBlackCompositeOp
            }
            CompositeOperator::CopyBlue => crate::bindings::CompositeOperator_CopyBlueCompositeOp,
            CompositeOperator::Copy => crate::bindings::CompositeOperator_CopyCompositeOp,
            CompositeOperator::CopyCyan => crate::bindings::CompositeOperator_CopyCyanCompositeOp,
            CompositeOperator::CopyGreen => crate::bindings::CompositeOperator_CopyGreenCompositeOp,
            CompositeOperator::CopyMagenta => {
                crate::bindings::CompositeOperator_CopyMagentaCompositeOp
            }
            CompositeOperator::CopyAlpha => crate::bindings::CompositeOperator_CopyAlphaCompositeOp,
            CompositeOperator::CopyRed => crate::bindings::CompositeOperator_CopyRedCompositeOp,
            CompositeOperator::CopyYellow => {
                crate::bindings::CompositeOperator_CopyYellowCompositeOp
            }
            CompositeOperator::Darken => crate::bindings::CompositeOperator_DarkenCompositeOp,
            CompositeOperator::DarkenIntensity => {
                crate::bindings::CompositeOperator_DarkenIntensityCompositeOp
            }
            CompositeOperator::Difference => {
                crate::bindings::CompositeOperator_DifferenceCompositeOp
            }
            CompositeOperator::Displace => crate::bindings::CompositeOperator_DisplaceCompositeOp,
            CompositeOperator::Dissolve => crate::bindings::CompositeOperator_DissolveCompositeOp,
            CompositeOperator::Distort => crate::bindings::CompositeOperator_DistortCompositeOp,
            CompositeOperator::DivideDst => crate::bindings::CompositeOperator_DivideDstCompositeOp,
            CompositeOperator::DivideSrc => crate::bindings::CompositeOperator_DivideSrcCompositeOp,
            CompositeOperator::DstAtop => crate::bindings::CompositeOperator_DstAtopCompositeOp,
            CompositeOperator::Dst => crate::bindings::CompositeOperator_DstCompositeOp,
            CompositeOperator::DstIn => crate::bindings::CompositeOperator_DstInCompositeOp,
            CompositeOperator::DstOut => crate::bindings::CompositeOperator_DstOutCompositeOp,
            CompositeOperator::DstOver => crate::bindings::CompositeOperator_DstOverCompositeOp,
            CompositeOperator::Exclusion => crate::bindings::CompositeOperator_ExclusionCompositeOp,
            CompositeOperator::HardLight => crate::bindings::CompositeOperator_HardLightCompositeOp,
            CompositeOperator::HardMix => crate::bindings::CompositeOperator_HardMixCompositeOp,
            CompositeOperator::Hue => crate::bindings::CompositeOperator_HueCompositeOp,
            CompositeOperator::In => crate::bindings::CompositeOperator_InCompositeOp,
            CompositeOperator::Intensity => crate::bindings::CompositeOperator_IntensityCompositeOp,
            CompositeOperator::Lighten => crate::bindings::CompositeOperator_LightenCompositeOp,
            CompositeOperator::LightenIntensity => {
                crate::bindings::CompositeOperator_LightenIntensityCompositeOp
            }
            CompositeOperator::LinearBurn => {
                crate::bindings::CompositeOperator_LinearBurnCompositeOp
            }
            CompositeOperator::LinearDodge => {
                crate::bindings::CompositeOperator_LinearDodgeCompositeOp
            }
            CompositeOperator::LinearLight => {
                crate::bindings::CompositeOperator_LinearLightCompositeOp
            }
            CompositeOperator::Luminize => crate::bindings::CompositeOperator_LuminizeCompositeOp,
            CompositeOperator::Mathematics => {
                crate::bindings::CompositeOperator_MathematicsCompositeOp
            }
            CompositeOperator::MinusDst => crate::bindings::CompositeOperator_MinusDstCompositeOp,
            CompositeOperator::MinusSrc => crate::bindings::CompositeOperator_MinusSrcCompositeOp,
            CompositeOperator::Modulate => crate::bindings::CompositeOperator_ModulateCompositeOp,
            CompositeOperator::ModulusAdd => {
                crate::bindings::CompositeOperator_ModulusAddCompositeOp
            }
            CompositeOperator::ModulusSubtract => {
                crate::bindings::CompositeOperator_ModulusSubtractCompositeOp
            }
            CompositeOperator::Multiply => crate::bindings::CompositeOperator_MultiplyCompositeOp,
            CompositeOperator::No => crate::bindings::CompositeOperator_NoCompositeOp,
            CompositeOperator::Out => crate::bindings::CompositeOperator_OutCompositeOp,
            CompositeOperator::Over => crate::bindings::CompositeOperator_OverCompositeOp,
            CompositeOperator::Overlay => crate::bindings::CompositeOperator_OverlayCompositeOp,
            CompositeOperator::PegtopLight => {
                crate::bindings::CompositeOperator_PegtopLightCompositeOp
            }
            CompositeOperator::PinLight => crate::bindings::CompositeOperator_PinLightCompositeOp,
            CompositeOperator::Plus => crate::bindings::CompositeOperator_PlusCompositeOp,
            CompositeOperator::Replace => crate::bindings::CompositeOperator_ReplaceCompositeOp,
            CompositeOperator::Saturate => crate::bindings::CompositeOperator_SaturateCompositeOp,
            CompositeOperator::Screen => crate::bindings::CompositeOperator_ScreenCompositeOp,
            CompositeOperator::SoftLight => crate::bindings::CompositeOperator_SoftLightCompositeOp,
            CompositeOperator::SrcAtop => crate::bindings::CompositeOperator_SrcAtopCompositeOp,
            CompositeOperator::Src => crate::bindings::CompositeOperator_SrcCompositeOp,
            CompositeOperator::SrcIn => crate::bindings::CompositeOperator_SrcInCompositeOp,
            CompositeOperator::SrcOut => crate::bindings::CompositeOperator_SrcOutCompositeOp,
            CompositeOperator::SrcOver => crate::bindings::CompositeOperator_SrcOverCompositeOp,
            CompositeOperator::Threshold => crate::bindings::CompositeOperator_ThresholdCompositeOp,
            CompositeOperator::VividLight => {
                crate::bindings::CompositeOperator_VividLightCompositeOp
            }
            CompositeOperator::Xor => crate::bindings::CompositeOperator_XorCompositeOp,
            CompositeOperator::Stereo => crate::bindings::CompositeOperator_StereoCompositeOp,
            CompositeOperator::Freeze => crate::bindings::CompositeOperator_FreezeCompositeOp,
            CompositeOperator::Interpolate => {
                crate::bindings::CompositeOperator_InterpolateCompositeOp
            }
            CompositeOperator::Negate => crate::bindings::CompositeOperator_NegateCompositeOp,
            CompositeOperator::Reflect => crate::bindings::CompositeOperator_ReflectCompositeOp,
            CompositeOperator::SoftBurn => crate::bindings::CompositeOperator_SoftBurnCompositeOp,
            CompositeOperator::SoftDodge => crate::bindings::CompositeOperator_SoftDodgeCompositeOp,
            CompositeOperator::Stamp => crate::bindings::CompositeOperator_StampCompositeOp,
            CompositeOperator::RMSE => crate::bindings::CompositeOperator_RMSECompositeOp,
            CompositeOperator::SaliencyBlend => {
                crate::bindings::CompositeOperator_SaliencyBlendCompositeOp
            }
            CompositeOperator::SeamlessBlend => {
                crate::bindings::CompositeOperator_SeamlessBlendCompositeOp
            }
        }
    }
}