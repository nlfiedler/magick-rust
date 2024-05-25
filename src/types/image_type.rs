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
pub enum ImageType {
    Undefined = bindings::ImageType_UndefinedType,
    Bilevel = bindings::ImageType_BilevelType,
    Grayscale = bindings::ImageType_GrayscaleType,
    GrayscaleAlpha = bindings::ImageType_GrayscaleAlphaType,
    Palette = bindings::ImageType_PaletteType,
    PaletteAlpha = bindings::ImageType_PaletteAlphaType,
    TrueColor = bindings::ImageType_TrueColorType,
    TrueColorAlpha = bindings::ImageType_TrueColorAlphaType,
    ColorSeparation = bindings::ImageType_ColorSeparationType,
    ColorSeparationAlpha = bindings::ImageType_ColorSeparationAlphaType,
    Optimize = bindings::ImageType_OptimizeType,
    PaletteBilevelAlpha = bindings::ImageType_PaletteBilevelAlphaType,
}

impl Default for ImageType {
    fn default() -> Self {
        return ImageType::Undefined;
    }
}

impl From<ImageType> for bindings::ImageType {
    fn from(value: ImageType) -> Self {
        return value as bindings::ImageType;
    }
}

impl From<bindings::ImageType> for ImageType {
    fn from(value: bindings::ImageType) -> Self {
        /*
         * SAFETY:
         *
         * `ImageType` has the same repr as `bindings::ImageType` - u32
         *
         * If `value` is less than SouthEast than it is in the vaild range and can be safely
         * reinterpreted as `ImageType`
         */
        if value <= bindings::ImageType_PaletteBilevelAlphaType {
            return unsafe { std::mem::transmute(value) };
        }
        return ImageType::default();
    }
}
