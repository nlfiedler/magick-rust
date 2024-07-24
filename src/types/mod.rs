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

mod geometry_info;
mod image;
mod kernel;

pub use bindings::AlignType;
pub use bindings::AlphaChannelOption;
pub use bindings::AutoThresholdMethod;
pub use bindings::ChannelType;
pub use bindings::ClipPathUnits;
pub use bindings::CompositeOperator;
pub use bindings::ColorspaceType;
pub use bindings::CompressionType;
pub use bindings::DecorationType;
pub use bindings::DirectionType;
pub use bindings::DisposeType;
pub use bindings::DitherMethod;
pub use bindings::EndianType;
pub use bindings::FillRule;
pub use bindings::FilterType;
pub use self::geometry_info::GeometryInfo;
pub use bindings::GravityType;
pub use self::image::Image;
pub use bindings::ImageType;
pub use bindings::InterlaceType;
pub use bindings::KernelInfoType;
pub use self::kernel::{KernelBuilder, KernelInfo};
pub use bindings::LayerMethod;
pub use bindings::LineCap;
pub use bindings::LineJoin;
pub use bindings::MagickEvaluateOperator;
pub use bindings::MagickFunction;
pub use bindings::MetricType;
pub use bindings::MorphologyMethod;
pub use bindings::OrientationType;
pub use bindings::PixelInterpolateMethod;
pub use bindings::PixelMask;
pub use bindings::RenderingIntent;
pub use bindings::ResolutionType;
pub use bindings::ResourceType;
pub use bindings::StatisticType;
pub use bindings::StretchType;
pub use bindings::StyleType;
