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
macro_rules! wand_common {
    (   $wand:ident,
        $new_wand:ident, $clear_wand:ident, $is_wand:ident, $clone:ident, $destroy:ident,
        $clear_exc:ident, $get_exc_type:ident, $get_exc:ident
    ) => {
        pub struct $wand {
            wand: *mut crate::bindings::$wand,
        }

        impl Default for $wand {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $wand {
            pub fn new() -> Self {
                $wand {
                    wand: unsafe { crate::bindings::$new_wand() },
                }
            }

            pub (crate) fn from_ptr(ptr: *mut crate::bindings::$wand) -> Self {
                $wand {
                    wand: ptr
                }
            }

            pub (crate) fn as_ptr(&self) -> *mut crate::bindings::$wand {
                self.wand
            }

            fn clear(&mut self) {
                unsafe { crate::bindings::$clear_wand(self.wand) }
            }

            pub fn clear_exception(&mut self) -> Result<()> {
                match unsafe { crate::bindings::$clear_exc(self.wand) } {
                    crate::bindings::MagickBooleanType::MagickTrue => Ok(()),
                    _ => Err(MagickError(
                        concat!("failed to clear", stringify!($wand), "exception").to_string(),
                    )),
                }
            }

            pub fn get_exception_type(&self) -> crate::bindings::ExceptionType {
                unsafe { crate::bindings::$get_exc_type(self.wand) }
            }

            pub fn get_exception(&self) -> Result<(String, crate::bindings::ExceptionType)> {
                let mut severity: crate::bindings::ExceptionType =
                    crate::bindings::ExceptionType::UndefinedException;

                let ptr = unsafe { crate::bindings::$get_exc(self.wand, &mut severity as *mut _) };
                if ptr.is_null() {
                    Err(MagickError(
                        concat!("null ptr returned by", stringify!($wand), "get_exception")
                            .to_string(),
                    ))
                } else {
                    let c_str = unsafe { CStr::from_ptr(ptr) };
                    let exception = c_str.to_string_lossy().into_owned();
                    unsafe { crate::bindings::RelinquishMagickMemory(ptr as *mut ::libc::c_void) };
                    Ok((exception, severity))
                }
            }

            pub fn is_wand(&self) -> Result<()> {
                match unsafe { crate::bindings::$is_wand(self.wand) } {
                    crate::bindings::MagickBooleanType::MagickTrue => Ok(()),
                    _ => Err(MagickError(
                        concat!(stringify!($wand), " not a wand").to_string(),
                    )),
                }
            }
        }

        impl Clone for $wand {
            fn clone(&self) -> Self {
                $wand {
                    wand: unsafe { crate::bindings::$clone(self.wand) },
                }
            }
        }

        impl Drop for $wand {
            fn drop(&mut self) {
                unsafe {
                    crate::bindings::$clear_exc(self.wand);
                    crate::bindings::$destroy(self.wand);
                }
            }
        }

        // The wand types should be safe to drop in a different thread
        unsafe impl Send for $wand {}

        // Probably shouldn't implement Sync because some methods might not be
        // safe to call on the same wand from different threads.
        // unsafe impl Sync for $wand {}
    };
}

macro_rules! get {
    ($($get:ident, $c_get:ident, $typ:ty )*) => {
        $(
            pub fn $get(&self) -> $typ {
                unsafe { crate::bindings::$c_get(self.wand).into() }
            }
        )*
    }
}

macro_rules! set_get {
    ($($get:ident, $set:ident, $c_get:ident, $c_set:ident, $typ:ty )*) => {
        $(
            pub fn $get(&self) -> $typ {
                unsafe { crate::bindings::$c_get(self.wand).into() }
            }
            pub fn $set(&mut self, v: $typ) -> Result<()> {
                match unsafe { crate::bindings::$c_set(self.wand, v.into()) } {
                    crate::bindings::MagickBooleanType::MagickTrue => Ok(()),
                    _ => Err(MagickError(concat!(stringify!($set), " returned false").to_string()))
                }
            }
        )*
        pub fn fmt_checked_settings(&self, f: &mut ::std::fmt::Formatter, prefix: &str) -> ::std::fmt::Result {
            $( writeln!(f, "{}{:<50}: {:?}", prefix, stringify!($c_get), self.$get())?; )*
            Ok(())
        }
    }
}

macro_rules! set_get_unchecked {
    ($($get:ident, $set:ident, $c_get:ident, $c_set:ident, $typ:ty )*) => {
        $(
            pub fn $get(&self) -> $typ {
                unsafe { crate::bindings::$c_get(self.wand).into() }
            }
            pub fn $set(&mut self, v: $typ) {
                unsafe { crate::bindings::$c_set(self.wand, v.into()) }
            }
        )*
        pub fn fmt_unchecked_settings(&self, f: &mut ::std::fmt::Formatter, prefix: &str) -> ::std::fmt::Result {
            $( writeln!(f, "{}{:<50}: {:?}", prefix, stringify!($c_get), self.$get())?; )*
            Ok(())
        }
    }
}

macro_rules! string_get {
    ($get:ident, $c_get:ident) => {
        pub fn $get(&self) -> Result<String> {
            let ptr = unsafe { crate::bindings::$c_get(self.wand) };
            if ptr.is_null() {
                Err(MagickError(
                    concat!("null ptr returned by ", stringify!($get)).to_string(),
                ))
            } else {
                let c_str = unsafe { ::std::ffi::CStr::from_ptr(ptr) };
                let result: String = c_str.to_string_lossy().into_owned();
                unsafe { crate::bindings::free(ptr as *mut ::libc::c_void) };
                Ok(result)
            }
        }
    };
}

macro_rules! string_set_get {
    ($($get:ident, $set:ident, $c_get:ident, $c_set:ident)*) => {
        $(
            string_get!($get, $c_get);
            pub fn $set(&mut self, s: &str) -> Result<()> {
                let c_string = std::ffi::CString::new(s).map_err(|_| "could not convert to cstring")?;
                match unsafe { crate::bindings::$c_set(self.wand, c_string.as_ptr()) } {
                    crate::bindings::MagickBooleanType::MagickTrue => Ok(()),
                    _ => Err(MagickError(concat!(stringify!($set), " returned false").to_string()))
                }
            }
        )*
        pub fn fmt_string_settings(&self, f: &mut ::std::fmt::Formatter, prefix: &str) -> ::std::fmt::Result {
            $( writeln!(f, "{}{:<50}: {:?}", prefix, stringify!($c_get), self.$get())?; )*
            Ok(())
        }
    }
}

macro_rules! string_set_get_unchecked {
    ($($get:ident, $set:ident, $c_get:ident, $c_set:ident )*) => {
        $(
            string_get!($get, $c_get);
            pub fn $set(&mut self, s: &str) -> Result<()> {
                let c_string = ::std::ffi::CString::new(s).map_err(|_| "could not convert to cstring")?;
                unsafe { crate::bindings::$c_set(self.wand, c_string.as_ptr()) };
                Ok(())
            }
        )*
        pub fn fmt_string_unchecked_settings(&self, f: &mut ::std::fmt::Formatter, prefix: &str) -> ::std::fmt::Result {
            $( writeln!(f, "{}{:<50}: {:?}", prefix, stringify!($c_get), self.$get())?; )*
            Ok(())
        }
    }
}

macro_rules! pixel_set_get {
    ($($get:ident, $set:ident, $c_get:ident, $c_set:ident )*) => {
        $(
            pub fn $get(&self) -> crate::PixelWand {
                let pw = crate::PixelWand::new();
                unsafe { crate::bindings::$c_get(self.wand, pw.as_ptr()) };
                pw
            }
            pub fn $set(&mut self, pw: &crate::PixelWand) {
                unsafe { crate::bindings::$c_set(self.wand, pw.as_ptr()) }
            }
        )*
        pub fn fmt_pixel_settings(&self, f: &mut ::std::fmt::Formatter, prefix: &str) -> ::std::fmt::Result {
            $(
                writeln!(f, "{}{:<50}: ", prefix, stringify!($c_get))?;
                self.$get().fmt_w_prefix(f, &format!("{}{:<53}", prefix, " ") )?;
            )*
            Ok(())
        }
    }
}

macro_rules! color_set_get {
    ($(
        $get:ident,   $get_quantum:ident,   $set:ident,   $set_quantum:ident,
        $c_get:ident, $c_get_quantum:ident, $c_set:ident, $c_set_quantum:ident
    )*) => {
        $(
            pub fn $get(&self) -> f64 {
                unsafe { crate::bindings::$c_get(self.wand) }
            }
            pub fn $get_quantum(&self) -> bindings::Quantum {
                unsafe { crate::bindings::$c_get_quantum(self.wand) }
            }
            pub fn $set(&mut self, v: f64) {
                unsafe { crate::bindings::$c_set(self.wand, v) }
            }
            pub fn $set_quantum(&mut self, v: bindings::Quantum) {
                unsafe { crate::bindings::$c_set_quantum(self.wand, v) }
            }
        )*
        pub fn fmt_color_settings(&self, f: &mut ::std::fmt::Formatter, prefix: &str) -> ::std::fmt::Result {
            writeln!(f, "{}Color: {:?}, normalized: {:?}\n{}hsl: {:?}",
                     prefix,
                     self.get_color_as_string(),
                     self.get_color_as_normalized_string(),
                     prefix,
                     self.get_hsl()
            )?;
            $( writeln!(f, "{}{:<10}: {:>} quantum: {}", prefix, stringify!($c_get).split_at(8).1, self.$get(), self.$get_quantum())?; )*
            Ok(())
        }
    }
}

macro_rules! mutations {
    ($($(#[$attr:meta])* $c_fun:ident => $fun:ident($($arg:ident: $ty:ty),*))*) => {
        $(
            $(#[$attr])*
            pub fn $fun(&self $(, $arg: $ty)*) -> Result<()> {
                match unsafe { bindings::$c_fun(self.wand $(, $arg.into())*) } {
                    bindings::MagickBooleanType::MagickTrue => Ok(()),
                    _ => Err(MagickError(concat!(stringify!($c_fun), " invocation failed").to_string()))
                }
            }
        )*
    }
}
