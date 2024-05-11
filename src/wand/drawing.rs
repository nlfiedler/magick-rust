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
use std::ffi::{CStr, CString};
use std::fmt;

use bindings;

use crate::result::MagickError;
use crate::result::Result;
use crate::{
    ClipPathUnits,
    FillRule,
    GravityType,
    StretchType,
    StyleType,
};

wand_common!(
    DrawingWand,
    NewDrawingWand,
    ClearDrawingWand,
    IsDrawingWand,
    CloneDrawingWand,
    DestroyDrawingWand,
    DrawClearException,
    DrawGetExceptionType,
    DrawGetException
);

impl DrawingWand {
    pub fn draw_annotation(&mut self, x: f64, y: f64, text: &str) -> Result<()> {
        let c_string = CString::new(text).map_err(|_| "could not convert to cstring")?;
        unsafe { bindings::DrawAnnotation(self.wand, x, y, c_string.as_ptr() as *const _) };
        Ok(())
    }

    pub fn draw_circle(&mut self, ox: f64, oy: f64, px: f64, py: f64) {
        unsafe {
            bindings::DrawCircle(self.wand, ox, oy, px, py);
        }
    }

    pub fn draw_rectangle(
        &mut self,
        upper_left_x: f64,
        upper_left_y: f64,
        lower_right_x: f64,
        lower_right_y: f64,
    ) {
        unsafe {
            bindings::DrawRectangle(
                self.wand,
                upper_left_x,
                upper_left_y,
                lower_right_x,
                lower_right_y,
            );
        }
    }

    string_set_get!(
        get_font,                   set_font,                     DrawGetFont,                  DrawSetFont
        get_font_family,            set_font_family,              DrawGetFontFamily,            DrawSetFontFamily
        get_vector_graphics,        set_vector_graphics,          DrawGetVectorGraphics,        DrawSetVectorGraphics
        get_clip_path,              set_clip_path,                DrawGetClipPath,              DrawSetClipPath
    );

    string_set_get_unchecked!(
        get_text_encoding,
        set_text_encoding,
        DrawGetTextEncoding,
        DrawSetTextEncoding
    );

    pixel_set_get!(
        get_border_color,           set_border_color,             DrawGetBorderColor,           DrawSetBorderColor
        get_fill_color,             set_fill_color,               DrawGetFillColor,             DrawSetFillColor
        get_stroke_color,           set_stroke_color,             DrawGetStrokeColor,           DrawSetStrokeColor
        get_text_under_color,       set_text_under_color,         DrawGetTextUnderColor,        DrawSetTextUnderColor
    );

    set_get_unchecked!(
        get_gravity,                set_gravity,                  DrawGetGravity,               DrawSetGravity,               GravityType
        get_opacity,                set_opacity,                  DrawGetOpacity,               DrawSetOpacity,               f64
        get_clip_rule,              set_clip_rule,                DrawGetClipRule,              DrawSetClipRule,              FillRule
        get_clip_units,             set_clip_units,               DrawGetClipUnits,             DrawSetClipUnits,             ClipPathUnits
        get_fill_rule,              set_fill_rule,                DrawGetFillRule,              DrawSetFillRule,              FillRule
        get_fill_opacity,           set_fill_opacity,             DrawGetFillOpacity,           DrawSetFillOpacity,           f64

        get_font_size,              set_font_size,                DrawGetFontSize,              DrawSetFontSize,              f64
        get_font_style,             set_font_style,               DrawGetFontStyle,             DrawSetFontStyle,             StyleType
        get_font_weight,            set_font_weight,              DrawGetFontWeight,            DrawSetFontWeight,            usize
        get_font_stretch,           set_font_stretch,             DrawGetFontStretch,           DrawSetFontStretch,           StretchType

        get_stroke_dash_offset,     set_stroke_dash_offset,       DrawGetStrokeDashOffset,      DrawSetStrokeDashOffset,      f64
        get_stroke_line_cap,        set_stroke_line_cap,          DrawGetStrokeLineCap,         DrawSetStrokeLineCap,         bindings::LineCap
        get_stroke_line_join,       set_stroke_line_join,         DrawGetStrokeLineJoin,        DrawSetStrokeLineJoin,        bindings::LineJoin
        get_stroke_miter_limit,     set_stroke_miter_limit,       DrawGetStrokeMiterLimit,      DrawSetStrokeMiterLimit,      usize
        get_stroke_opacity,         set_stroke_opacity,           DrawGetStrokeOpacity,         DrawSetStrokeOpacity,         f64
        get_stroke_width,           set_stroke_width,             DrawGetStrokeWidth,           DrawSetStrokeWidth,           f64
        get_stroke_antialias,       set_stroke_antialias,         DrawGetStrokeAntialias,       DrawSetStrokeAntialias,       bindings::MagickBooleanType

        get_text_alignment,         set_text_alignment,           DrawGetTextAlignment,         DrawSetTextAlignment,         bindings::AlignType
        get_text_antialias,         set_text_antialias,           DrawGetTextAntialias,         DrawSetTextAntialias,         bindings::MagickBooleanType
        get_text_decoration,        set_text_decoration,          DrawGetTextDecoration,        DrawSetTextDecoration,        bindings::DecorationType
        get_text_direction,         set_text_direction,           DrawGetTextDirection,         DrawSetTextDirection,         bindings::DirectionType
        get_text_kerning,           set_text_kerning,             DrawGetTextKerning,           DrawSetTextKerning,           f64
        get_text_interline_spacing, set_text_interline_spacing,   DrawGetTextInterlineSpacing,  DrawSetTextInterlineSpacing,  f64
        get_text_interword_spacing, set_text_interword_spacing,   DrawGetTextInterwordSpacing,  DrawSetTextInterwordSpacing,  f64
    );
}

impl fmt::Debug for DrawingWand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "DrawingWand {{")?;
        writeln!(f, "    Exception: {:?}", self.get_exception())?;
        writeln!(f, "    IsWand: {:?}", self.is_wand())?;
        self.fmt_unchecked_settings(f, "    ")?;
        self.fmt_string_settings(f, "    ")?;
        self.fmt_string_unchecked_settings(f, "    ")?;
        self.fmt_pixel_settings(f, "    ")?;
        writeln!(f, "}}")
    }
}
