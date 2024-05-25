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
pub enum FilterType {
    Undefined = bindings::FilterType_UndefinedFilter,
    Point = bindings::FilterType_PointFilter,
    Box = bindings::FilterType_BoxFilter,
    Triangle = bindings::FilterType_TriangleFilter,
    Hermite = bindings::FilterType_HermiteFilter,
    Hann = bindings::FilterType_HannFilter,
    Hamming = bindings::FilterType_HammingFilter,
    Blackman = bindings::FilterType_BlackmanFilter,
    Gaussian = bindings::FilterType_GaussianFilter,
    Quadratic = bindings::FilterType_QuadraticFilter,
    Cubic = bindings::FilterType_CubicFilter,
    Catrom = bindings::FilterType_CatromFilter,
    Mitchell = bindings::FilterType_MitchellFilter,
    Jinc = bindings::FilterType_JincFilter,
    Sinc = bindings::FilterType_SincFilter,
    SincFast = bindings::FilterType_SincFastFilter,
    Kaiser = bindings::FilterType_KaiserFilter,
    Welch = bindings::FilterType_WelchFilter,
    Parzen = bindings::FilterType_ParzenFilter,
    Bohman = bindings::FilterType_BohmanFilter,
    Bartlett = bindings::FilterType_BartlettFilter,
    Lagrange = bindings::FilterType_LagrangeFilter,
    Lanczos = bindings::FilterType_LanczosFilter,
    LanczosSharp = bindings::FilterType_LanczosSharpFilter,
    Lanczos2 = bindings::FilterType_Lanczos2Filter,
    Lanczos2Sharp = bindings::FilterType_Lanczos2SharpFilter,
    Robidoux = bindings::FilterType_RobidouxFilter,
    RobidouxSharp = bindings::FilterType_RobidouxSharpFilter,
    Cosine = bindings::FilterType_CosineFilter,
    Spline = bindings::FilterType_SplineFilter,
    LanczosRadius = bindings::FilterType_LanczosRadiusFilter,
    CubicSpline = bindings::FilterType_CubicSplineFilter,
    Sentinel = bindings::FilterType_SentinelFilter,
}

impl From<FilterType> for bindings::FilterType {
    fn from(value: FilterType) -> Self {
        return value as bindings::FilterType;
    }
}
