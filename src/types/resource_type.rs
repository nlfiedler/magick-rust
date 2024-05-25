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

/// Resource type to use with [set_resource_limit](crate::MagickWand::set_resource_limit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Undefined = bindings::ResourceType_UndefinedResource as isize,
    Area = bindings::ResourceType_AreaResource as isize,
    Disk = bindings::ResourceType_DiskResource as isize,
    File = bindings::ResourceType_FileResource as isize,
    Height = bindings::ResourceType_HeightResource as isize,
    Map = bindings::ResourceType_MapResource as isize,
    Memory = bindings::ResourceType_MemoryResource as isize,
    Thread = bindings::ResourceType_ThreadResource as isize,
    Throttle = bindings::ResourceType_ThrottleResource as isize,
    Time = bindings::ResourceType_TimeResource as isize,
    Width = bindings::ResourceType_WidthResource as isize,
    ListLength = bindings::ResourceType_ListLengthResource as isize,
}

impl From<ResourceType> for bindings::ResourceType {
    fn from(value: ResourceType) -> Self {
        return value as bindings::ResourceType;
    }
}
