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
pub enum AutoThresholdMethod {
    Undefined = bindings::AutoThresholdMethod_UndefinedThresholdMethod,
    Kapur = bindings::AutoThresholdMethod_KapurThresholdMethod,
    OTSU = bindings::AutoThresholdMethod_OTSUThresholdMethod,
    Triangle = bindings::AutoThresholdMethod_TriangleThresholdMethod,
}

impl Default for AutoThresholdMethod {
    fn default() -> Self {
        return AutoThresholdMethod::Undefined;
    }
}

impl From<AutoThresholdMethod> for bindings::AutoThresholdMethod {
    fn from(value: AutoThresholdMethod) -> Self {
        return value as bindings::AutoThresholdMethod;
    }
}
