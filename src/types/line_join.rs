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
pub enum LineJoin {
    Undefined = bindings::LineJoin_UndefinedJoin,
    Miter = bindings::LineJoin_MiterJoin,
    Round = bindings::LineJoin_RoundJoin,
    Bevel = bindings::LineJoin_BevelJoin,
}

impl Default for LineJoin {
    fn default() -> Self {
        return LineJoin::Undefined;
    }
}

impl From<LineJoin> for bindings::LineJoin {
    fn from(value: LineJoin) -> Self {
        return value as bindings::LineJoin;
    }
}

impl From<bindings::LineJoin> for LineJoin {
    fn from(value: bindings::LineJoin) -> Self {
        /*
         * SAFETY:
         *
         * `LineJoin` has the same repr as `bindings::LineJoin` - u32
         *
         * If `value` is less than Bevel than it is in the vaild range and can be safely
         * reinterpreted as `LineJoin`
         */
        if value <= bindings::LineJoin_BevelJoin {
            return unsafe { std::mem::transmute(value) };
        }
        return LineJoin::default();
    }
}
