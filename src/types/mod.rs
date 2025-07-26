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

pub use crate::bindings::AlignType;
pub use crate::bindings::AlphaChannelOption;
pub use crate::bindings::AutoThresholdMethod;
pub use crate::bindings::ChannelType;
pub use crate::bindings::ClipPathUnits;
pub use crate::bindings::CompositeOperator;
pub use crate::bindings::ColorspaceType;
pub use crate::bindings::CompressionType;
pub use crate::bindings::DecorationType;
pub use crate::bindings::DirectionType;
pub use crate::bindings::DisposeType;
pub use crate::bindings::DitherMethod;
pub use crate::bindings::EndianType;
pub use crate::bindings::FillRule;
pub use crate::bindings::FilterType;
pub use self::geometry_info::GeometryInfo;
pub use crate::bindings::GravityType;
pub use self::image::Image;
pub use crate::bindings::ImageType;
pub use crate::bindings::InterlaceType;
pub use crate::bindings::KernelInfoType;
pub use self::kernel::{KernelBuilder, KernelInfo};
pub use crate::bindings::LayerMethod;
pub use crate::bindings::LineCap;
pub use crate::bindings::LineJoin;
pub use crate::bindings::MagickEvaluateOperator;
pub use crate::bindings::MagickFunction;
pub use crate::bindings::MetricType;
pub use crate::bindings::MorphologyMethod;
pub use crate::bindings::OrientationType;
pub use crate::bindings::PixelInterpolateMethod;
pub use crate::bindings::PixelMask;
pub use crate::bindings::RenderingIntent;
pub use crate::bindings::ResolutionType;
pub use crate::bindings::ResourceType;
pub use crate::bindings::StatisticType;
pub use crate::bindings::StretchType;
pub use crate::bindings::StyleType;
