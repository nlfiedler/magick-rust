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
pub enum StretchType {
    Undefined = bindings::StretchType_UndefinedStretch,
    Normal = bindings::StretchType_NormalStretch,
    UltraCondensed = bindings::StretchType_UltraCondensedStretch,
    ExtraCondensed = bindings::StretchType_ExtraCondensedStretch,
    Condensed = bindings::StretchType_CondensedStretch,
    SemiCondensed = bindings::StretchType_SemiCondensedStretch,
    SemiExpanded = bindings::StretchType_SemiExpandedStretch,
    Expanded = bindings::StretchType_ExpandedStretch,
    ExtraExpanded = bindings::StretchType_ExtraExpandedStretch,
    UltraExpanded = bindings::StretchType_UltraExpandedStretch,
    Any = bindings::StretchType_AnyStretch,
}

impl Default for StretchType {
    fn default() -> Self {
        return StretchType::Undefined;
    }
}

impl From<StretchType> for bindings::StretchType {
    fn from(value: StretchType) -> Self {
        return value as bindings::StretchType;
    }
}

impl From<bindings::StretchType> for StretchType {
    fn from(value: bindings::StretchType) -> Self {
        /*
         * SAFETY:
         *
         * `StretchType` has the same repr as `bindings::StretchType` - u32
         *
         * If `value` is less than Any than it is in the vaild range and can be safely
         * reinterpreted as `StretchType`
         */
        if value <= bindings::StretchType_AnyStretch {
            return unsafe { std::mem::transmute(value) };
        }
        return StretchType::default();
    }
}
