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
pub enum OrientationType {
    Undefined = bindings::OrientationType_UndefinedOrientation,
    TopLeft = bindings::OrientationType_TopLeftOrientation,
    TopRight = bindings::OrientationType_TopRightOrientation,
    BottomRight = bindings::OrientationType_BottomRightOrientation,
    BottomLeft = bindings::OrientationType_BottomLeftOrientation,
    LeftTop = bindings::OrientationType_LeftTopOrientation,
    RightTop = bindings::OrientationType_RightTopOrientation,
    RightBottom = bindings::OrientationType_RightBottomOrientation,
    LeftBottom = bindings::OrientationType_LeftBottomOrientation,
}

impl Default for OrientationType {
    fn default() -> Self {
        return OrientationType::Undefined;
    }
}

impl From<OrientationType> for bindings::OrientationType {
    fn from(value: OrientationType) -> Self {
        return value as bindings::OrientationType;
    }
}

impl From<bindings::OrientationType> for OrientationType {
    fn from(value: bindings::OrientationType) -> Self {
        /*
         * SAFETY:
         *
         * `OrientationType` has the same repr as `bindings::OrientationType` - u32
         *
         * If `value` is less than LeftBottom than it is in the vaild range and can be safely
         * reinterpreted as `OrientationType`
         */
        if value <= bindings::OrientationType_LeftBottomOrientation {
            return unsafe { std::mem::transmute(value) };
        }
        return OrientationType::default();
    }
}
