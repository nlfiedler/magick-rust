use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AlphaChannelOption {
    Undefined    = bindings::AlphaChannelOption_UndefinedAlphaChannel,
    Activate     = bindings::AlphaChannelOption_ActivateAlphaChannel,
    Associate    = bindings::AlphaChannelOption_AssociateAlphaChannel,
    Background   = bindings::AlphaChannelOption_BackgroundAlphaChannel,
    Copy         = bindings::AlphaChannelOption_CopyAlphaChannel,
    Deactivate   = bindings::AlphaChannelOption_DeactivateAlphaChannel,
    Discrete     = bindings::AlphaChannelOption_DiscreteAlphaChannel,
    Disassociate = bindings::AlphaChannelOption_DisassociateAlphaChannel,
    Extract      = bindings::AlphaChannelOption_ExtractAlphaChannel,
    Off          = bindings::AlphaChannelOption_OffAlphaChannel,
    On           = bindings::AlphaChannelOption_OnAlphaChannel,
    Opaque       = bindings::AlphaChannelOption_OpaqueAlphaChannel,
    Remove       = bindings::AlphaChannelOption_RemoveAlphaChannel,
    Set          = bindings::AlphaChannelOption_SetAlphaChannel,
    Shape        = bindings::AlphaChannelOption_ShapeAlphaChannel,
    Transparent  = bindings::AlphaChannelOption_TransparentAlphaChannel,
    OffIfOpaque  = bindings::AlphaChannelOption_OffIfOpaqueAlphaChannel,
}

impl Default for AlphaChannelOption {
    fn default() -> Self {
        return AlphaChannelOption::Undefined;
    }
}

impl From<AlphaChannelOption> for bindings::AlphaChannelOption {
    fn from(value: AlphaChannelOption) -> Self {
        return value as bindings::AlphaChannelOption;
    }
}

impl From<bindings::AlphaChannelOption> for AlphaChannelOption {
    fn from(value: bindings::AlphaChannelOption) -> Self {
        /*
         * SAFETY:
         *
         * `AlphaChannelOption` has the same repr as `bindings::AlphaChannelOption` - u32
         *
         * If `value` is less than OffIfOpaque than it is in the vaild range and can be safely
         * reinterpreted as `AlphaChannelOption`
         */
        if value <= bindings::AlphaChannelOption_OffIfOpaqueAlphaChannel {
            return unsafe { std::mem::transmute(value) };
        }
        return AlphaChannelOption::default();
    }
}
