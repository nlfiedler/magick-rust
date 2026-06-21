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

pub use self::geometry_info::GeometryInfo;
pub use self::image::{Image, ImageMut, ImageRef, Images, ImagesMut};
pub use self::kernel::{KernelBuilder, KernelInfo};
/// Text alignment for drawing operations (Left, Center, Right).
pub use crate::bindings::AlignType;
/// How an image's alpha (transparency) channel is activated or modified.
pub use crate::bindings::AlphaChannelOption;
/// Algorithm used to automatically choose a thresholding value (Kapur, OTSU, Triangle).
pub use crate::bindings::AutoThresholdMethod;
/// Selects which image channels an operation applies to (Red, Green, Blue, Alpha, etc.).
pub use crate::bindings::ChannelType;
/// Coordinate system used to interpret a clip path's units.
pub use crate::bindings::ClipPathUnits;
/// The colorspace of an image (RGB, sRGB, CMYK, Gray, etc.).
pub use crate::bindings::ColorspaceType;
/// How a source image is combined with a destination image when compositing (Over, Multiply, etc.).
pub use crate::bindings::CompositeOperator;
/// Compression algorithm used when encoding an image (LZW, JPEG, Zip, etc.).
pub use crate::bindings::CompressionType;
/// Text decoration applied when drawing (Underline, Overline, LineThrough).
pub use crate::bindings::DecorationType;
/// Text reading direction for drawing (left-to-right or right-to-left).
pub use crate::bindings::DirectionType;
/// How a frame is disposed of before the next frame is displayed in an animation.
pub use crate::bindings::DisposeType;
/// Dithering algorithm used when reducing an image's color count (Riemersma, Floyd-Steinberg).
pub use crate::bindings::DitherMethod;
/// Byte order (endianness) used when reading or writing image data.
pub use crate::bindings::EndianType;
/// Rule determining which regions of a path are considered "inside" when filling (EvenOdd, NonZero).
pub use crate::bindings::FillRule;
/// Resampling filter used when resizing or distorting an image (Lanczos, Point, etc.).
pub use crate::bindings::FilterType;
/// Placement of content relative to a bounding region (NorthWest, Center, etc.).
pub use crate::bindings::GravityType;
/// The type of an image (Bilevel, Grayscale, Palette, TrueColor, etc.).
pub use crate::bindings::ImageType;
/// Interlacing scheme used when encoding an image (None, Line, Plane, JPEG, etc.).
pub use crate::bindings::InterlaceType;
/// The kind of built-in convolution/morphology kernel to construct (Gaussian, Laplacian, etc.).
pub use crate::bindings::KernelInfoType;
/// Layer operation applied to a multi-frame image (Merge, Flatten, Coalesce, etc.).
pub use crate::bindings::LayerMethod;
/// Shape drawn at the open ends of a stroked line (Butt, Round, Square).
pub use crate::bindings::LineCap;
/// Shape drawn where two stroked line segments meet (Miter, Round, Bevel).
pub use crate::bindings::LineJoin;
/// Arithmetic operator applied per pixel by the evaluate operations (Add, Multiply, etc.).
pub use crate::bindings::MagickEvaluateOperator;
/// Mathematical function applied per pixel (Polynomial, Sinusoid, Arcsin, etc.).
pub use crate::bindings::MagickFunction;
/// Metric used to compare two images (MSE, RMSE, PSNR, etc.).
pub use crate::bindings::MetricType;
/// Morphology operation applied with a kernel (Erode, Dilate, Open, Close, etc.).
pub use crate::bindings::MorphologyMethod;
/// The EXIF-style orientation of an image (TopLeft, BottomRight, etc.).
pub use crate::bindings::OrientationType;
/// How a flood-fill or paint operation matches neighboring pixels (Point, Replace, Floodfill, etc.).
pub use crate::bindings::PaintMethod;
/// Interpolation method used when sampling between pixels (Bilinear, Nearest, Catrom, etc.).
pub use crate::bindings::PixelInterpolateMethod;
/// Identifies a per-pixel mask channel (Read, Write, Composite).
pub use crate::bindings::PixelMask;
/// Color rendering intent for ICC profile conversion (Perceptual, Saturation, Relative, Absolute).
pub use crate::bindings::RenderingIntent;
/// Unit of measurement for an image's resolution (PixelsPerInch, PixelsPerCentimeter).
pub use crate::bindings::ResolutionType;
/// A resource whose usage ImageMagick can limit (Memory, Disk, Threads, etc.).
pub use crate::bindings::ResourceType;
/// Statistical operation applied over a pixel neighborhood (Median, Mean, Gradient, etc.).
pub use crate::bindings::StatisticType;
/// Font stretch (width) used when drawing text (Normal, Condensed, Expanded, etc.).
pub use crate::bindings::StretchType;
/// Font style used when drawing text (Normal, Italic, Oblique).
pub use crate::bindings::StyleType;
/// How pixels outside the image bounds are computed during sampling (Edge, Tile, Mirror, etc.).
pub use crate::bindings::VirtualPixelMethod;
