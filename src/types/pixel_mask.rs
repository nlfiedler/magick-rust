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
pub enum PixelMask {
    Undefined = bindings::PixelMask_UndefinedPixelMask,
    Read = bindings::PixelMask_ReadPixelMask,
    Write = bindings::PixelMask_WritePixelMask,
    Composite = bindings::PixelMask_CompositePixelMask,
}

impl Default for PixelMask {
    fn default() -> Self {
        return PixelMask::Undefined;
    }
}

impl From<PixelMask> for bindings::PixelMask {
    fn from(value: PixelMask) -> Self {
        return value as bindings::PixelMask;
    }
}
