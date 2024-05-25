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
pub enum InterlaceType {
    Undefined = bindings::InterlaceType_UndefinedInterlace,
    No = bindings::InterlaceType_NoInterlace,
    Line = bindings::InterlaceType_LineInterlace,
    Plane = bindings::InterlaceType_PlaneInterlace,
    Partition = bindings::InterlaceType_PartitionInterlace,
    GIF = bindings::InterlaceType_GIFInterlace,
    JPEG = bindings::InterlaceType_JPEGInterlace,
    PNG = bindings::InterlaceType_PNGInterlace,
}

impl Default for InterlaceType {
    fn default() -> Self {
        return InterlaceType::Undefined;
    }
}

impl From<InterlaceType> for bindings::InterlaceType {
    fn from(value: InterlaceType) -> Self {
        return value as bindings::InterlaceType;
    }
}

impl From<bindings::InterlaceType> for InterlaceType {
    fn from(value: bindings::InterlaceType) -> Self {
        /*
         * SAFETY:
         *
         * `InterlaceType` has the same repr as `bindings::InterlaceType` - u32
         *
         * If `value` is less than PNG than it is in the vaild range and can be safely
         * reinterpreted as `InterlaceType`
         */
        if value <= bindings::InterlaceType_PNGInterlace {
            return unsafe { std::mem::transmute(value) };
        }
        return InterlaceType::default();
    }
}
