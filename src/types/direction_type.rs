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
pub enum DirectionType {
    Undefined = bindings::DirectionType_UndefinedDirection,
    RightToLeft = bindings::DirectionType_RightToLeftDirection,
    LeftToRight = bindings::DirectionType_LeftToRightDirection,
    TopToBottom = bindings::DirectionType_TopToBottomDirection,
}

impl Default for DirectionType {
    fn default() -> Self {
        return DirectionType::Undefined;
    }
}

impl From<DirectionType> for bindings::DirectionType {
    fn from(value: DirectionType) -> Self {
        return value as bindings::DirectionType;
    }
}

impl From<bindings::DirectionType> for DirectionType {
    fn from(value: bindings::DirectionType) -> Self {
        /*
         * SAFETY:
         *
         * `DirectionType` has the same repr as `bindings::DirectionType` - u32
         *
         * If `value` is less than TopToBottom than it is in the vaild range and can be safely
         * reinterpreted as `DirectionType`
         */
        if value <= bindings::DirectionType_TopToBottomDirection {
            return unsafe { std::mem::transmute(value) };
        }
        return DirectionType::default();
    }
}
