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
use super::bindings;

pub trait FromRust<T> {
    fn from_rust(t: T) -> Self;
}

impl FromRust<bool> for bindings::MagickBooleanType {
    fn from_rust(b: bool) -> Self {
        if b {
            bindings::MagickTrue
        } else {
            bindings::MagickFalse
        }
    }
}

pub trait ToMagick<T> {
    fn to_magick(self) -> T;
}

impl<T, E> ToMagick<T> for E where T: FromRust<E> {
    fn to_magick(self) -> T {
        <T as FromRust<E>>::from_rust(self)
    }
}
