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
pub enum DisposeType {
    /*
     * Identical to `Undefined`
     */
    // Unrecognized = bindings::DisposeType_UnrecognizedDispose,
    Undefined = bindings::DisposeType_UndefinedDispose,
    None = bindings::DisposeType_NoneDispose,
    Background = bindings::DisposeType_BackgroundDispose,
    Previous = bindings::DisposeType_PreviousDispose,
}

impl Default for DisposeType {
    fn default() -> Self {
        return DisposeType::Undefined;
    }
}

impl From<DisposeType> for bindings::DisposeType {
    fn from(value: DisposeType) -> Self {
        return value as bindings::DisposeType;
    }
}

impl From<bindings::DisposeType> for DisposeType {
    fn from(value: bindings::DisposeType) -> Self {
        /*
         * SAFETY:
         *
         * `DisposeType` has the same repr as `bindings::DisposeType` - u32
         *
         * If `value` is less than Previous than it is in the vaild range and can be safely
         * reinterpreted as `DisposeType`
         */
        if value <= bindings::DisposeType_PreviousDispose {
            return unsafe { std::mem::transmute(value) };
        }
        return DisposeType::default();
    }
}
