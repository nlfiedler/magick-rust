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
pub enum FillRule {
    Undefined = bindings::FillRule_UndefinedRule,
    EvenOdd = bindings::FillRule_EvenOddRule,
    NonZero = bindings::FillRule_NonZeroRule,
}

impl Default for FillRule {
    fn default() -> Self {
        return FillRule::Undefined;
    }
}

impl From<FillRule> for bindings::FillRule {
    fn from(value: FillRule) -> Self {
        return value as bindings::FillRule;
    }
}

impl From<bindings::FillRule> for FillRule {
    fn from(value: bindings::FillRule) -> Self {
        /*
         * SAFETY:
         *
         * `FillRule` has the same repr as `bindings::FillRule` - u32
         *
         * If `value` is less than NonZero than it is in the vaild range and can be safely
         * reinterpreted as `FillRule`
         */
        if value <= bindings::FillRule_NonZeroRule {
            return unsafe { std::mem::transmute(value) };
        }
        return FillRule::default();
    }
}
