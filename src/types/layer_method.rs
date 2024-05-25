/*
 * Copyright 2024 5ohue
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LayerMethod {
    Undefined = bindings::LayerMethod_UndefinedLayer,
    Coalesce = bindings::LayerMethod_CoalesceLayer,
    CompareAny = bindings::LayerMethod_CompareAnyLayer,
    CompareClear = bindings::LayerMethod_CompareClearLayer,
    CompareOverlay = bindings::LayerMethod_CompareOverlayLayer,
    Dispose = bindings::LayerMethod_DisposeLayer,
    Optimize = bindings::LayerMethod_OptimizeLayer,
    OptimizeImage = bindings::LayerMethod_OptimizeImageLayer,
    OptimizePlus = bindings::LayerMethod_OptimizePlusLayer,
    OptimizeTrans = bindings::LayerMethod_OptimizeTransLayer,
    RemoveDups = bindings::LayerMethod_RemoveDupsLayer,
    RemoveZero = bindings::LayerMethod_RemoveZeroLayer,
    Composite = bindings::LayerMethod_CompositeLayer,
    Merge = bindings::LayerMethod_MergeLayer,
    Flatten = bindings::LayerMethod_FlattenLayer,
    Mosaic = bindings::LayerMethod_MosaicLayer,
    TrimBounds = bindings::LayerMethod_TrimBoundsLayer,
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
