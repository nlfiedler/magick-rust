pub enum CompositeOperator {
    UndefinedCompositeOp,
    AlphaCompositeOp,
    AtopCompositeOp,
    BlendCompositeOp,
    BlurCompositeOp,
    BumpmapCompositeOp,
    ChangeMaskCompositeOp,
    ClearCompositeOp,
    ColorBurnCompositeOp,
    ColorDodgeCompositeOp,
    ColorizeCompositeOp,
    CopyBlackCompositeOp,
    CopyBlueCompositeOp,
    CopyCompositeOp,
    CopyCyanCompositeOp,
    CopyGreenCompositeOp,
    CopyMagentaCompositeOp,
    CopyAlphaCompositeOp,
    CopyRedCompositeOp,
    CopyYellowCompositeOp,
    DarkenCompositeOp,
    DarkenIntensityCompositeOp,
    DifferenceCompositeOp,
    DisplaceCompositeOp,
    DissolveCompositeOp,
    DistortCompositeOp,
    DivideDstCompositeOp,
    DivideSrcCompositeOp,
    DstAtopCompositeOp,
    DstCompositeOp,
    DstInCompositeOp,
    DstOutCompositeOp,
    DstOverCompositeOp,
    ExclusionCompositeOp,
    HardLightCompositeOp,
    HardMixCompositeOp,
    HueCompositeOp,
    InCompositeOp,
    IntensityCompositeOp,
    LightenCompositeOp,
    LightenIntensityCompositeOp,
    LinearBurnCompositeOp,
    LinearDodgeCompositeOp,
    LinearLightCompositeOp,
    LuminizeCompositeOp,
    MathematicsCompositeOp,
    MinusDstCompositeOp,
    MinusSrcCompositeOp,
    ModulateCompositeOp,
    ModulusAddCompositeOp,
    ModulusSubtractCompositeOp,
    MultiplyCompositeOp,
    NoCompositeOp,
    OutCompositeOp,
    OverCompositeOp,
    OverlayCompositeOp,
    PegtopLightCompositeOp,
    PinLightCompositeOp,
    PlusCompositeOp,
    ReplaceCompositeOp,
    SaturateCompositeOp,
    ScreenCompositeOp,
    SoftLightCompositeOp,
    SrcAtopCompositeOp,
    SrcCompositeOp,
    SrcInCompositeOp,
    SrcOutCompositeOp,
    SrcOverCompositeOp,
    ThresholdCompositeOp,
    VividLightCompositeOp,
    XorCompositeOp,
    StereoCompositeOp,
    FreezeCompositeOp,
    InterpolateCompositeOp,
    NegateCompositeOp,
    ReflectCompositeOp,
    SoftBurnCompositeOp,
    SoftDodgeCompositeOp,
    StampCompositeOp,
    RMSECompositeOp,
    SaliencyBlendCompositeOp,
    SeamlessBlendCompositeOp,
}

impl From<CompositeOperator> for crate::bindings::CompositeOperator {
    fn from(value: CompositeOperator) -> Self {
        match value {
            CompositeOperator::UndefinedCompositeOp => {
                crate::bindings::CompositeOperator_UndefinedCompositeOp
            }
            CompositeOperator::AlphaCompositeOp => {
                crate::bindings::CompositeOperator_AlphaCompositeOp
            }
            CompositeOperator::AtopCompositeOp => {
                crate::bindings::CompositeOperator_AtopCompositeOp
            }
            CompositeOperator::BlendCompositeOp => {
                crate::bindings::CompositeOperator_BlendCompositeOp
            }
            CompositeOperator::BlurCompositeOp => {
                crate::bindings::CompositeOperator_BlurCompositeOp
            }
            CompositeOperator::BumpmapCompositeOp => {
                crate::bindings::CompositeOperator_BumpmapCompositeOp
            }
            CompositeOperator::ChangeMaskCompositeOp => {
                crate::bindings::CompositeOperator_ChangeMaskCompositeOp
            }
            CompositeOperator::ClearCompositeOp => {
                crate::bindings::CompositeOperator_ClearCompositeOp
            }
            CompositeOperator::ColorBurnCompositeOp => {
                crate::bindings::CompositeOperator_ColorBurnCompositeOp
            }
            CompositeOperator::ColorDodgeCompositeOp => {
                crate::bindings::CompositeOperator_ColorDodgeCompositeOp
            }
            CompositeOperator::ColorizeCompositeOp => { crate::bindings::CompositeOperator_ColorizeCompositeOp
            }
            CompositeOperator::CopyBlackCompositeOp => {
                crate::bindings::CompositeOperator_CopyBlackCompositeOp
            }
            CompositeOperator::CopyBlueCompositeOp => {
                crate::bindings::CompositeOperator_CopyBlueCompositeOp
            }
            CompositeOperator::CopyCompositeOp => {
                crate::bindings::CompositeOperator_CopyCompositeOp
            }
            CompositeOperator::CopyCyanCompositeOp => {
                crate::bindings::CompositeOperator_CopyCyanCompositeOp
            }
            CompositeOperator::CopyGreenCompositeOp => {
                crate::bindings::CompositeOperator_CopyGreenCompositeOp
            }
            CompositeOperator::CopyMagentaCompositeOp => {
                crate::bindings::CompositeOperator_CopyMagentaCompositeOp
            }
            CompositeOperator::CopyAlphaCompositeOp => {
                crate::bindings::CompositeOperator_CopyAlphaCompositeOp
            }
            CompositeOperator::CopyRedCompositeOp => {
                crate::bindings::CompositeOperator_CopyRedCompositeOp
            }
            CompositeOperator::CopyYellowCompositeOp => {
                crate::bindings::CompositeOperator_CopyYellowCompositeOp
            }
            CompositeOperator::DarkenCompositeOp => {
                crate::bindings::CompositeOperator_DarkenCompositeOp
            }
            CompositeOperator::DarkenIntensityCompositeOp => {
                crate::bindings::CompositeOperator_DarkenIntensityCompositeOp
            }
            CompositeOperator::DifferenceCompositeOp => {
                crate::bindings::CompositeOperator_DifferenceCompositeOp
            }
            CompositeOperator::DisplaceCompositeOp => {
                crate::bindings::CompositeOperator_DisplaceCompositeOp
            }
            CompositeOperator::DissolveCompositeOp => {
                crate::bindings::CompositeOperator_DissolveCompositeOp
            }
            CompositeOperator::DistortCompositeOp => {
                crate::bindings::CompositeOperator_DistortCompositeOp
            }
            CompositeOperator::DivideDstCompositeOp => {
                crate::bindings::CompositeOperator_DivideDstCompositeOp
            }
            CompositeOperator::DivideSrcCompositeOp => {
                crate::bindings::CompositeOperator_DivideSrcCompositeOp
            }
            CompositeOperator::DstAtopCompositeOp => {
                crate::bindings::CompositeOperator_DstAtopCompositeOp
            }
            CompositeOperator::DstCompositeOp => crate::bindings::CompositeOperator_DstCompositeOp,
            CompositeOperator::DstInCompositeOp => {
                crate::bindings::CompositeOperator_DstInCompositeOp
            }
            CompositeOperator::DstOutCompositeOp => {
                crate::bindings::CompositeOperator_DstOutCompositeOp
            }
            CompositeOperator::DstOverCompositeOp => {
                crate::bindings::CompositeOperator_DstOverCompositeOp
            }
            CompositeOperator::ExclusionCompositeOp => {
                crate::bindings::CompositeOperator_ExclusionCompositeOp
            }
            CompositeOperator::HardLightCompositeOp => {
                crate::bindings::CompositeOperator_HardLightCompositeOp
            }
            CompositeOperator::HardMixCompositeOp => {
                crate::bindings::CompositeOperator_HardMixCompositeOp
            }
            CompositeOperator::HueCompositeOp => crate::bindings::CompositeOperator_HueCompositeOp,
            CompositeOperator::InCompositeOp => crate::bindings::CompositeOperator_InCompositeOp,
            CompositeOperator::IntensityCompositeOp => {
                crate::bindings::CompositeOperator_IntensityCompositeOp
            }
            CompositeOperator::LightenCompositeOp => {
                crate::bindings::CompositeOperator_LightenCompositeOp
            }
            CompositeOperator::LightenIntensityCompositeOp => {
                crate::bindings::CompositeOperator_LightenIntensityCompositeOp
            }
            CompositeOperator::LinearBurnCompositeOp => {
                crate::bindings::CompositeOperator_LinearBurnCompositeOp
            }
            CompositeOperator::LinearDodgeCompositeOp => {
                crate::bindings::CompositeOperator_LinearDodgeCompositeOp
            }
            CompositeOperator::LinearLightCompositeOp => {
                crate::bindings::CompositeOperator_LinearLightCompositeOp
            }
            CompositeOperator::LuminizeCompositeOp => {
                crate::bindings::CompositeOperator_LuminizeCompositeOp
            }
            CompositeOperator::MathematicsCompositeOp => {
                crate::bindings::CompositeOperator_MathematicsCompositeOp
            }
            CompositeOperator::MinusDstCompositeOp => {
                crate::bindings::CompositeOperator_MinusDstCompositeOp
            }
            CompositeOperator::MinusSrcCompositeOp => {
                crate::bindings::CompositeOperator_MinusSrcCompositeOp
            }
            CompositeOperator::ModulateCompositeOp => {
                crate::bindings::CompositeOperator_ModulateCompositeOp
            }
            CompositeOperator::ModulusAddCompositeOp => {
                crate::bindings::CompositeOperator_ModulusAddCompositeOp
            }
            CompositeOperator::ModulusSubtractCompositeOp => {
                crate::bindings::CompositeOperator_ModulusSubtractCompositeOp
            }
            CompositeOperator::MultiplyCompositeOp => {
                crate::bindings::CompositeOperator_MultiplyCompositeOp
            }
            CompositeOperator::NoCompositeOp => crate::bindings::CompositeOperator_NoCompositeOp,
            CompositeOperator::OutCompositeOp => crate::bindings::CompositeOperator_OutCompositeOp,
            CompositeOperator::OverCompositeOp => {
                crate::bindings::CompositeOperator_OverCompositeOp
            }
            CompositeOperator::OverlayCompositeOp => {
                crate::bindings::CompositeOperator_OverlayCompositeOp
            }
            CompositeOperator::PegtopLightCompositeOp => {
                crate::bindings::CompositeOperator_PegtopLightCompositeOp
            }
            CompositeOperator::PinLightCompositeOp => {
                crate::bindings::CompositeOperator_PinLightCompositeOp
            }
            CompositeOperator::PlusCompositeOp => {
                crate::bindings::CompositeOperator_PlusCompositeOp
            }
            CompositeOperator::ReplaceCompositeOp => {
                crate::bindings::CompositeOperator_ReplaceCompositeOp
            }
            CompositeOperator::SaturateCompositeOp => {
                crate::bindings::CompositeOperator_SaturateCompositeOp
            }
            CompositeOperator::ScreenCompositeOp => {
                crate::bindings::CompositeOperator_ScreenCompositeOp
            }
            CompositeOperator::SoftLightCompositeOp => {
                crate::bindings::CompositeOperator_SoftLightCompositeOp
            }
            CompositeOperator::SrcAtopCompositeOp => {
                crate::bindings::CompositeOperator_SrcAtopCompositeOp
            }
            CompositeOperator::SrcCompositeOp => crate::bindings::CompositeOperator_SrcCompositeOp,
            CompositeOperator::SrcInCompositeOp => {
                crate::bindings::CompositeOperator_SrcInCompositeOp
            }
            CompositeOperator::SrcOutCompositeOp => {
                crate::bindings::CompositeOperator_SrcOutCompositeOp
            }
            CompositeOperator::SrcOverCompositeOp => {
                crate::bindings::CompositeOperator_SrcOverCompositeOp
            }
            CompositeOperator::ThresholdCompositeOp => {
                crate::bindings::CompositeOperator_ThresholdCompositeOp
            }
            CompositeOperator::VividLightCompositeOp => {
                crate::bindings::CompositeOperator_VividLightCompositeOp
            }
            CompositeOperator::XorCompositeOp => crate::bindings::CompositeOperator_XorCompositeOp,
            CompositeOperator::StereoCompositeOp => {
                crate::bindings::CompositeOperator_StereoCompositeOp
            }
            CompositeOperator::FreezeCompositeOp => {
                crate::bindings::CompositeOperator_FreezeCompositeOp
            }
            CompositeOperator::InterpolateCompositeOp => {
                crate::bindings::CompositeOperator_InterpolateCompositeOp
            }
            CompositeOperator::NegateCompositeOp => {
                crate::bindings::CompositeOperator_NegateCompositeOp
            }
            CompositeOperator::ReflectCompositeOp => {
                crate::bindings::CompositeOperator_ReflectCompositeOp
            }
            CompositeOperator::SoftBurnCompositeOp => {
                crate::bindings::CompositeOperator_SoftBurnCompositeOp
            }
            CompositeOperator::SoftDodgeCompositeOp => {
                crate::bindings::CompositeOperator_SoftDodgeCompositeOp
            }
            CompositeOperator::StampCompositeOp => {
                crate::bindings::CompositeOperator_StampCompositeOp
            }
            CompositeOperator::RMSECompositeOp => {
                crate::bindings::CompositeOperator_RMSECompositeOp
            }
            CompositeOperator::SaliencyBlendCompositeOp => {
                crate::bindings::CompositeOperator_SaliencyBlendCompositeOp
            }
            CompositeOperator::SeamlessBlendCompositeOp => {
                crate::bindings::CompositeOperator_SeamlessBlendCompositeOp
            }
        }
    }
}
