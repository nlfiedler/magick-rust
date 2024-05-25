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
pub enum RenderingIntent {
    Undefined = bindings::RenderingIntent_UndefinedIntent,
    Saturation = bindings::RenderingIntent_SaturationIntent,
    Perceptual = bindings::RenderingIntent_PerceptualIntent,
    Absolute = bindings::RenderingIntent_AbsoluteIntent,
    Relative = bindings::RenderingIntent_RelativeIntent,
}

impl Default for RenderingIntent {
    fn default() -> Self {
        return RenderingIntent::Undefined;
    }
}

impl From<RenderingIntent> for bindings::RenderingIntent {
    fn from(value: RenderingIntent) -> Self {
        return value as bindings::RenderingIntent;
    }
}

impl From<bindings::RenderingIntent> for RenderingIntent {
    fn from(value: bindings::RenderingIntent) -> Self {
        /*
         * SAFETY:
         *
         * `RenderingIntent` has the same repr as `bindings::RenderingIntent` - u32
         *
         * If `value` is less than Relative than it is in the vaild range and can be safely
         * reinterpreted as `RenderingIntent`
         */
        if value <= bindings::RenderingIntent_RelativeIntent {
            return unsafe { std::mem::transmute(value) };
        }
        return RenderingIntent::default();
    }
}
