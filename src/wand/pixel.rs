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

use crate::bindings;
use crate::result::MagickError;

use super::MagickTrue;
use crate::result::Result;

#[derive(Default, Debug)]
pub struct HSL {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
}

wand_common!(
    PixelWand,
    NewPixelWand,
    ClearPixelWand,
    IsPixelWand,
    ClonePixelWand,
    DestroyPixelWand,
    PixelClearException,
    PixelGetExceptionType,
    PixelGetException
);

impl PixelWand {
    pub fn is_similar(&self, other: &PixelWand, fuzz: f64) -> bool {
        let result = unsafe { bindings::IsPixelWandSimilar(self.wand, other.wand, fuzz) };
        return result == MagickTrue;
    }

    pub fn get_hsl(&self) -> HSL {
        let mut hsl = HSL::default();
        unsafe {
            bindings::PixelGetHSL(
                self.wand,
                &mut hsl.hue as *mut _,
                &mut hsl.saturation as *mut _,
                &mut hsl.lightness as *mut _,
            );
        }
        hsl
    }

    pub fn set_hsl(&self, hsl: &HSL) {
        unsafe {
            bindings::PixelSetHSL(self.wand, hsl.hue, hsl.saturation, hsl.lightness);
        }
    }

    pub fn fmt_w_prefix(&self, f: &mut fmt::Formatter, prefix: &str) -> fmt::Result {
        let mut prf = prefix.to_string();
        prf.push_str("    ");
        writeln!(f, "{}PixelWand {{", prefix)?;
        writeln!(f, "{}Exception: {:?}", prf, self.get_exception())?;
        writeln!(f, "{}IsWand: {:?}", prf, self.is_wand())?;
        self.fmt_unchecked_settings(f, &prf)?;
        self.fmt_color_settings(f, &prf)?;
        writeln!(f, "{}}}", prefix)
    }

    pub fn set_color(&mut self, s: &str) -> Result<()> {
        let c_string = CString::new(s).map_err(|_| "could not convert to cstring")?;
        match unsafe { bindings::PixelSetColor(self.wand, c_string.as_ptr()) } {
            MagickTrue => Ok(()),
            _ => Err(MagickError(self.get_exception()?.0)),
        }
    }

    string_get!(get_color_as_string, PixelGetColorAsString);
    string_get!(
        get_color_as_normalized_string,
        PixelGetColorAsNormalizedString
    );

    set_get_unchecked!(
        get_color_count, set_color_count, PixelGetColorCount, PixelSetColorCount,   usize
        get_index,       set_index,       PixelGetIndex,      PixelSetIndex,        bindings::Quantum
        get_fuzz,        set_fuzz,        PixelGetFuzz,       PixelSetFuzz,         f64
    );

    color_set_get!(
        get_alpha,        get_alpha_quantum,       set_alpha,        set_alpha_quantum,
        PixelGetAlpha,    PixelGetAlphaQuantum,    PixelSetAlpha,    PixelSetAlphaQuantum
        get_black,        get_black_quantum,       set_black,        set_black_quantum,
        PixelGetBlack,    PixelGetBlackQuantum,    PixelSetBlack,    PixelSetBlackQuantum
        get_blue,         get_blue_quantum,        set_blue,         set_blue_quantum,
        PixelGetBlue,     PixelGetBlueQuantum,     PixelSetBlue,     PixelSetBlueQuantum
        get_cyan,         get_cyan_quantum,        set_cyan,         set_cyan_quantum,
        PixelGetCyan,     PixelGetCyanQuantum,     PixelSetCyan,     PixelSetCyanQuantum
        get_green,        get_green_quantum,       set_green,        set_green_quantum,
        PixelGetGreen,    PixelGetGreenQuantum,    PixelSetGreen,    PixelSetGreenQuantum
        get_magenta,      get_magenta_quantum,     set_magenta,      set_magenta_quantum,
        PixelGetMagenta,  PixelGetMagentaQuantum,  PixelSetMagenta,  PixelSetMagentaQuantum
        get_red,          get_red_quantum,         set_red,          set_red_quantum,
        PixelGetRed,      PixelGetRedQuantum,      PixelSetRed,      PixelSetRedQuantum
        get_yellow,       get_yellow_quantum,      set_yellow,       set_yellow_quantum,
        PixelGetYellow,   PixelGetYellowQuantum,   PixelSetYellow,   PixelSetYellowQuantum
    );
}

impl fmt::Debug for PixelWand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_w_prefix(f, "")
    }
}
