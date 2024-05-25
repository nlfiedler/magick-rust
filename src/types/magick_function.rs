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
pub enum MagickFunction {
    Undefined = bindings::MagickFunction_UndefinedFunction,
    Arcsin = bindings::MagickFunction_ArcsinFunction,
    Arctan = bindings::MagickFunction_ArctanFunction,
    Polynomial = bindings::MagickFunction_PolynomialFunction,
    Sinusoid = bindings::MagickFunction_SinusoidFunction,
}

impl Default for MagickFunction {
    fn default() -> Self {
        return MagickFunction::Undefined;
    }
}

impl From<MagickFunction> for bindings::MagickFunction {
    fn from(value: MagickFunction) -> Self {
        return value as bindings::MagickFunction;
    }
}
