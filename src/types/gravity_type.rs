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
pub enum GravityType {
    Undefined = bindings::GravityType_UndefinedGravity,
    /*
     * Identical to `Undefined`
     */
    // Forget    = bindings::GravityType_ForgetGravity,
    NorthWest = bindings::GravityType_NorthWestGravity,
    North = bindings::GravityType_NorthGravity,
    NorthEast = bindings::GravityType_NorthEastGravity,
    West = bindings::GravityType_WestGravity,
    Center = bindings::GravityType_CenterGravity,
    East = bindings::GravityType_EastGravity,
    SouthWest = bindings::GravityType_SouthWestGravity,
    South = bindings::GravityType_SouthGravity,
    SouthEast = bindings::GravityType_SouthEastGravity,
}

impl Default for GravityType {
    fn default() -> Self {
        return GravityType::Undefined;
    }
}

impl From<GravityType> for bindings::GravityType {
    fn from(value: GravityType) -> Self {
        return value as bindings::GravityType;
    }
}

impl From<bindings::GravityType> for GravityType {
    fn from(value: bindings::GravityType) -> Self {
        /*
         * SAFETY:
         *
         * `GravityType` has the same repr as `bindings::GravityType` - u32
         *
         * If `value` is less than SouthEast than it is in the vaild range and can be safely
         * reinterpreted as `GravityType`
         */
        if value <= bindings::GravityType_SouthEastGravity {
            return unsafe { std::mem::transmute(value) };
        }
        return GravityType::default();
    }
}
