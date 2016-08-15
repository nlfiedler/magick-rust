/*
 * Copyright 2016 Mattis Marjak
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
use ::bindings;

pub enum FilterType {
    UndefinedFilter = bindings::UndefinedFilter as isize,
    PointFilter = bindings::PointFilter as isize,
    BoxFilter = bindings::BoxFilter as isize,
    TriangleFilter = bindings::TriangleFilter as isize,
    HermiteFilter = bindings::HermiteFilter as isize,
    HanningFilter = bindings::HanningFilter as isize,
    HammingFilter = bindings::HammingFilter as isize,
    BlackmanFilter = bindings::BlackmanFilter as isize,
    GaussianFilter = bindings::GaussianFilter as isize,
    QuadraticFilter = bindings::QuadraticFilter as isize,
    CubicFilter = bindings::CubicFilter as isize,
    CatromFilter = bindings::CatromFilter as isize,
    MitchellFilter = bindings::MitchellFilter as isize,
    JincFilter = bindings::JincFilter as isize,
    SincFilter = bindings::SincFilter as isize,
    SincFastFilter = bindings::SincFastFilter as isize,
    KaiserFilter = bindings::KaiserFilter as isize,
    WelshFilter = bindings::WelshFilter as isize,
    ParzenFilter = bindings::ParzenFilter as isize,
    BohmanFilter = bindings::BohmanFilter as isize,
    BartlettFilter = bindings::BartlettFilter as isize,
    LagrangeFilter = bindings::LagrangeFilter as isize,
    LanczosFilter = bindings::LanczosFilter as isize,
    LanczosSharpFilter = bindings::LanczosSharpFilter as isize,
    Lanczos2Filter = bindings::Lanczos2Filter as isize,
    Lanczos2SharpFilter = bindings::Lanczos2SharpFilter as isize,
    RobidouxFilter = bindings::RobidouxFilter as isize,
    RobidouxSharpFilter = bindings::RobidouxSharpFilter as isize,
    CosineFilter = bindings::CosineFilter as isize,
    SplineFilter = bindings::SplineFilter as isize,
    LanczosRadiusFilter = bindings::LanczosRadiusFilter as isize,
    SentinelFilter = bindings::SentinelFilter as isize
}
