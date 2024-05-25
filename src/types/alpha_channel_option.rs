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
pub enum AlphaChannelOption {
    Undefined = bindings::AlphaChannelOption_UndefinedAlphaChannel,
    Activate = bindings::AlphaChannelOption_ActivateAlphaChannel,
    Associate = bindings::AlphaChannelOption_AssociateAlphaChannel,
    Background = bindings::AlphaChannelOption_BackgroundAlphaChannel,
    Copy = bindings::AlphaChannelOption_CopyAlphaChannel,
    Deactivate = bindings::AlphaChannelOption_DeactivateAlphaChannel,
    Discrete = bindings::AlphaChannelOption_DiscreteAlphaChannel,
    Disassociate = bindings::AlphaChannelOption_DisassociateAlphaChannel,
    Extract = bindings::AlphaChannelOption_ExtractAlphaChannel,
    Off = bindings::AlphaChannelOption_OffAlphaChannel,
    On = bindings::AlphaChannelOption_OnAlphaChannel,
    Opaque = bindings::AlphaChannelOption_OpaqueAlphaChannel,
    Remove = bindings::AlphaChannelOption_RemoveAlphaChannel,
    Set = bindings::AlphaChannelOption_SetAlphaChannel,
    Shape = bindings::AlphaChannelOption_ShapeAlphaChannel,
    Transparent = bindings::AlphaChannelOption_TransparentAlphaChannel,
    OffIfOpaque = bindings::AlphaChannelOption_OffIfOpaqueAlphaChannel,
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
