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
pub enum AlignType {
    Undefined = bindings::AlignType_UndefinedAlign,
    Left = bindings::AlignType_LeftAlign,
    Center = bindings::AlignType_CenterAlign,
    Right = bindings::AlignType_RightAlign,
}

impl Default for AlignType {
    fn default() -> Self {
        return AlignType::Undefined;
    }
}

impl From<AlignType> for bindings::AlignType {
    fn from(value: AlignType) -> Self {
        return value as bindings::AlignType;
    }
}

impl From<bindings::AlignType> for AlignType {
    fn from(value: bindings::AlignType) -> Self {
        /*
         * SAFETY:
         *
         * `AlignType` has the same repr as `bindings::AlignType` - u32
         *
         * If `value` is less than Right than it is in the vaild range and can be safely
         * reinterpreted as `AlignType`
         */
        if value <= bindings::AlignType_RightAlign {
            return unsafe { std::mem::transmute(value) };
        }
        return AlignType::default();
    }
}
