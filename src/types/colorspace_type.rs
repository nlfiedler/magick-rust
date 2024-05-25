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
pub enum ColorspaceType {
    Undefined = bindings::ColorspaceType_UndefinedColorspace,
    CMY = bindings::ColorspaceType_CMYColorspace,
    CMYK = bindings::ColorspaceType_CMYKColorspace,
    GRAY = bindings::ColorspaceType_GRAYColorspace,
    HCL = bindings::ColorspaceType_HCLColorspace,
    HCLp = bindings::ColorspaceType_HCLpColorspace,
    HSB = bindings::ColorspaceType_HSBColorspace,
    HSI = bindings::ColorspaceType_HSIColorspace,
    HSL = bindings::ColorspaceType_HSLColorspace,
    HSV = bindings::ColorspaceType_HSVColorspace,
    HWB = bindings::ColorspaceType_HWBColorspace,
    Lab = bindings::ColorspaceType_LabColorspace,
    LCH = bindings::ColorspaceType_LCHColorspace,
    LCHab = bindings::ColorspaceType_LCHabColorspace,
    LCHuv = bindings::ColorspaceType_LCHuvColorspace,
    Log = bindings::ColorspaceType_LogColorspace,
    LMS = bindings::ColorspaceType_LMSColorspace,
    Luv = bindings::ColorspaceType_LuvColorspace,
    OHTA = bindings::ColorspaceType_OHTAColorspace,
    Rec601YCbCr = bindings::ColorspaceType_Rec601YCbCrColorspace,
    Rec709YCbCr = bindings::ColorspaceType_Rec709YCbCrColorspace,
    RGB = bindings::ColorspaceType_RGBColorspace,
    scRGB = bindings::ColorspaceType_scRGBColorspace,
    sRGB = bindings::ColorspaceType_sRGBColorspace,
    Transparent = bindings::ColorspaceType_TransparentColorspace,
    xyY = bindings::ColorspaceType_xyYColorspace,
    XYZ = bindings::ColorspaceType_XYZColorspace,
    YCbCr = bindings::ColorspaceType_YCbCrColorspace,
    YCC = bindings::ColorspaceType_YCCColorspace,
    YDbDr = bindings::ColorspaceType_YDbDrColorspace,
    YIQ = bindings::ColorspaceType_YIQColorspace,
    YPbPr = bindings::ColorspaceType_YPbPrColorspace,
    YUV = bindings::ColorspaceType_YUVColorspace,
    LinearGRAY = bindings::ColorspaceType_LinearGRAYColorspace,
    Jzazbz = bindings::ColorspaceType_JzazbzColorspace,
    DisplayP3 = bindings::ColorspaceType_DisplayP3Colorspace,
    Adobe98 = bindings::ColorspaceType_Adobe98Colorspace,
    ProPhoto = bindings::ColorspaceType_ProPhotoColorspace,
    Oklab = bindings::ColorspaceType_OklabColorspace,
    Oklch = bindings::ColorspaceType_OklchColorspace,
}

impl Default for ColorspaceType {
    fn default() -> Self {
        return ColorspaceType::RGB;
    }
}

impl From<ColorspaceType> for bindings::ColorspaceType {
    fn from(value: ColorspaceType) -> Self {
        return value as bindings::ColorspaceType;
    }
}

impl From<bindings::ColorspaceType> for ColorspaceType {
    fn from(value: bindings::ColorspaceType) -> Self {
        /*
         * SAFETY:
         *
         * `ColorspaceType` has the same repr as `bindings::ColorspaceType` - u32
         *
         * If `value` is less than Oklch than it is in the vaild range and can be safely
         * reinterpreted as `ColorspaceType`
         */
        if value <= bindings::ColorspaceType_OklchColorspace {
            return unsafe { std::mem::transmute(value) };
        }
        return ColorspaceType::default();
    }
}
