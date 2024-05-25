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
pub enum LineCap {
    Undefined = bindings::LineCap_UndefinedCap,
    Butt = bindings::LineCap_ButtCap,
    Round = bindings::LineCap_RoundCap,
    Square = bindings::LineCap_SquareCap,
}

impl Default for LineCap {
    fn default() -> Self {
        return LineCap::Undefined;
    }
}

impl From<LineCap> for bindings::LineCap {
    fn from(value: LineCap) -> Self {
        return value as bindings::LineCap;
    }
}

impl From<bindings::LineCap> for LineCap {
    fn from(value: bindings::LineCap) -> Self {
        /*
         * SAFETY:
         *
         * `LineCap` has the same repr as `bindings::LineCap` - u32
         *
         * If `value` is less than Square than it is in the vaild range and can be safely
         * reinterpreted as `LineCap`
         */
        if value <= bindings::LineCap_SquareCap {
            return unsafe { std::mem::transmute(value) };
        }
        return LineCap::default();
    }
}
