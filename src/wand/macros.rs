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
            pub wand: *mut ::bindings::$wand
        }

        impl $wand {
            pub fn new() -> Self {
                $wand {
                    wand: unsafe { ::bindings::$new_wand() }
                }
            }

            fn clear(&mut self) {
                unsafe { ::bindings::$clear_wand(self.wand) }
            }

            fn clear_exception(&mut self) -> Result<(), &'static str> {
                match unsafe { ::bindings::$clear_exc(self.wand) } {
                    ::bindings::MagickBooleanType::MagickTrue => Ok(()),
                    _ => Err(concat!("failed to clear", stringify!($wand), "exception"))
                }
            }

            fn get_exception_type(&self) -> ::bindings::ExceptionType {
                unsafe { ::bindings::$get_exc_type(self.wand) }
            }

            fn get_exception(&self) -> Result<(String, ::bindings::ExceptionType), &'static str> {
                let mut severity: ::bindings::ExceptionType = ::bindings::ExceptionType::UndefinedException;
                // TODO: memory management
                let ptr = unsafe { ::bindings::$get_exc(self.wand, &mut severity as *mut _) };
                if ptr.is_null() {
                    Err(concat!("null ptr returned by", stringify!($wand), "get_exception"))
                } else {
                    let c_str = unsafe { CStr::from_ptr(ptr) };
                    Ok((c_str.to_string_lossy().into_owned(), severity))
                }
            }

            pub fn is_wand(&self) -> Result<(), &'static str> {
                match unsafe { ::bindings::$is_wand(self.wand) } {
                    ::bindings::MagickBooleanType::MagickTrue => Ok(()),
                    _ => Err(concat!(stringify!($wand), " not a wand"))
                }
            }
        }

        impl Clone for $wand {
            fn clone(&self) -> Self {
                $wand {
                    wand: unsafe { ::bindings::$clone(self.wand) }
                }
            }
        }

        impl Drop for $wand {
            fn drop(&mut self) {
                unsafe {
                    ::bindings::$clear_exc(self.wand);
                    ::bindings::$destroy(self.wand);
                }
            }
        }
    }
}

macro_rules! get {
    ($($get:ident, $c_get:ident, $typ:ty )*) => {
        $(
            pub fn $get(&self) -> $typ {
                unsafe { ::bindings::$c_get(self.wand) }
            }
        )*
    }
}

macro_rules! set_get {
    ($($get:ident, $set:ident, $c_get:ident, $c_set:ident, $typ:ty )*) => {
        $(
            pub fn $get(&self) -> $typ {
                unsafe { ::bindings::$c_get(self.wand) }
            }
            pub fn $set(&mut self, v: $typ) -> Result<(), &'static str> {
                match unsafe { ::bindings::$c_set(self.wand, v) } {
                    ::bindings::MagickBooleanType::MagickTrue => Ok(()),
                    _ => Err(concat!(stringify!($set), " returned false"))
                }
            }
        )*
        pub fn fmt_checked_settings(&self, f: &mut ::std::fmt::Formatter, prefix: &str) -> ::std::fmt::Result {
            $( try!(writeln!(f, "{}{:<50}: {:?}", prefix, stringify!($c_get), self.$get())); )*
            Ok(())
        }
    }
}

macro_rules! set_get_unchecked {
    ($($get:ident, $set:ident, $c_get:ident, $c_set:ident, $typ:ty )*) => {
        $(
            pub fn $get(&self) -> $typ {
                unsafe { ::bindings::$c_get(self.wand) }
            }
            pub fn $set(&mut self, v: $typ) {
                unsafe { ::bindings::$c_set(self.wand, v) }
            }
        )*
        pub fn fmt_unchecked_settings(&self, f: &mut ::std::fmt::Formatter, prefix: &str) -> ::std::fmt::Result {
            $( try!(writeln!(f, "{}{:<50}: {:?}", prefix, stringify!($c_get), self.$get())); )*
            Ok(())
        }
    }
}

macro_rules! string_get {
    ($get:ident, $c_get:ident) => {
        pub fn $get(&self) -> Result<String, &'static str> {
            // TODO: memory management
            let ptr = unsafe { ::bindings::$c_get(self.wand) };
            if ptr.is_null() {
                Err(concat!("null ptr returned by ", stringify!($get)))
            } else {
                let c_str = unsafe { ::std::ffi::CStr::from_ptr(ptr) };
                Ok(c_str.to_string_lossy().into_owned())
            }
        }
    }
}

macro_rules! string_set_get {
    ($($get:ident, $set:ident, $c_get:ident, $c_set:ident)*) => {
        $(
            string_get!($get, $c_get);
            pub fn $set(&mut self, s: &str) -> Result<(), &'static str> {
                let c_string = try!(::std::ffi::CString::new(s).map_err(|_| "could not convert to cstring"));
                match unsafe { ::bindings::$c_set(self.wand, c_string.as_ptr()) } {
                    ::bindings::MagickBooleanType::MagickTrue => Ok(()),
                    _ => Err(concat!(stringify!($set), " returned false"))
                }
            }
        )*
        pub fn fmt_string_settings(&self, f: &mut ::std::fmt::Formatter, prefix: &str) -> ::std::fmt::Result {
            $( try!(writeln!(f, "{}{:<50}: {:?}", prefix, stringify!($c_get), self.$get())); )*
            Ok(())
        }
    }
}


macro_rules! string_set_get_unchecked {
    ($($get:ident, $set:ident, $c_get:ident, $c_set:ident )*) => {
        $(
            string_get!($get, $c_get);
            pub fn $set(&mut self, s: &str) -> Result<(), &'static str> {
                let c_string = try!(::std::ffi::CString::new(s).map_err(|_| "could not convert to cstring"));
                unsafe { ::bindings::$c_set(self.wand, c_string.as_ptr()) };
                Ok(())
            }
        )*
        pub fn fmt_string_unchecked_settings(&self, f: &mut ::std::fmt::Formatter, prefix: &str) -> ::std::fmt::Result {
            $( try!(writeln!(f, "{}{:<50}: {:?}", prefix, stringify!($c_get), self.$get())); )*
            Ok(())
        }
    }
}


macro_rules! pixel_set_get {
    ($($get:ident, $set:ident, $c_get:ident, $c_set:ident )*) => {
        $(
            pub fn $get(&self) -> ::PixelWand {
                let pw = ::PixelWand::new();
                unsafe { ::bindings::$c_get(self.wand, pw.wand) };
                pw
            }
            pub fn $set(&mut self, pw: &::PixelWand) {
                unsafe { ::bindings::$c_set(self.wand, pw.wand) }
            }
        )*
        pub fn fmt_pixel_settings(&self, f: &mut ::std::fmt::Formatter, prefix: &str) -> ::std::fmt::Result {
            $(
                try!(writeln!(f, "{}{:<50}: ", prefix, stringify!($c_get)));
                try!(self.$get().fmt_w_prefix(f, &format!("{}{:<53}", prefix, " ") ));
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
                unsafe { ::bindings::$c_get(self.wand) }
            }
            pub fn $get_quantum(&self) -> bindings::Quantum {
                unsafe { ::bindings::$c_get_quantum(self.wand) }
            }
            pub fn $set(&mut self, v: f64) {
                unsafe { ::bindings::$c_set(self.wand, v) }
            }
            pub fn $set_quantum(&mut self, v: bindings::Quantum) {
                unsafe { ::bindings::$c_set_quantum(self.wand, v) }
            }
        )*
        pub fn fmt_color_settings(&self, f: &mut ::std::fmt::Formatter, prefix: &str) -> ::std::fmt::Result {
            try!(writeln!(f, "{}Color: {:?}, normalized: {:?}\n{}hsl: {:?}",
                     prefix,
                     self.get_color_as_string(),
                     self.get_color_as_normalized_string(),
                     prefix,
                     self.get_hsl()
            ));
            $( try!(writeln!(f, "{}{:<10}: {:>} quantum: {}", prefix, stringify!($c_get).split_at(8).1, self.$get(), self.$get_quantum())); )*
            Ok(())
        }
    }
}

macro_rules! mutations {
    ($($(#[$attr:meta])* $c_fun:ident => $fun:ident($($arg:ident: $ty:ty),*))*) => {
        $(
            $(#[$attr])*
            pub fn $fun(&self $(, $arg: $ty)*) -> Result<(), &'static str> {
                match unsafe { bindings::$c_fun(self.wand $(, $arg)*) } {
                    bindings::MagickBooleanType::MagickTrue => Ok(()),
                    _ => Err(concat!(stringify!($c_fun), " invocation failed"))
                }
            }
        )*
    }
}
