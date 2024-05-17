use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LayerMethod {
    Undefined      = bindings::LayerMethod_UndefinedLayer,
    Coalesce       = bindings::LayerMethod_CoalesceLayer,
    CompareAny     = bindings::LayerMethod_CompareAnyLayer,
    CompareClear   = bindings::LayerMethod_CompareClearLayer,
    CompareOverlay = bindings::LayerMethod_CompareOverlayLayer,
    Dispose        = bindings::LayerMethod_DisposeLayer,
    Optimize       = bindings::LayerMethod_OptimizeLayer,
    OptimizeImage  = bindings::LayerMethod_OptimizeImageLayer,
    OptimizePlus   = bindings::LayerMethod_OptimizePlusLayer,
    OptimizeTrans  = bindings::LayerMethod_OptimizeTransLayer,
    RemoveDups     = bindings::LayerMethod_RemoveDupsLayer,
    RemoveZero     = bindings::LayerMethod_RemoveZeroLayer,
    Composite      = bindings::LayerMethod_CompositeLayer,
    Merge          = bindings::LayerMethod_MergeLayer,
    Flatten        = bindings::LayerMethod_FlattenLayer,
    Mosaic         = bindings::LayerMethod_MosaicLayer,
    TrimBounds     = bindings::LayerMethod_TrimBoundsLayer,
}

impl Default for LayerMethod {
    fn default() -> Self {
        return LayerMethod::Undefined;
    }
}

impl From<LayerMethod> for bindings::LayerMethod {
    fn from(value: LayerMethod) -> Self {
        return value as bindings::LayerMethod;
    }
}
