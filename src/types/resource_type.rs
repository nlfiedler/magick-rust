use std::convert::TryInto;

use crate::bindings;

/// Resource type to use with [set_resource_limit](crate::MagickWand::set_resource_limit)
#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Undefined  = bindings::ResourceType_UndefinedResource  as isize,
    Area       = bindings::ResourceType_AreaResource       as isize,
    Disk       = bindings::ResourceType_DiskResource       as isize,
    File       = bindings::ResourceType_FileResource       as isize,
    Height     = bindings::ResourceType_HeightResource     as isize,
    Map        = bindings::ResourceType_MapResource        as isize,
    Memory     = bindings::ResourceType_MemoryResource     as isize,
    Thread     = bindings::ResourceType_ThreadResource     as isize,
    Throttle   = bindings::ResourceType_ThrottleResource   as isize,
    Time       = bindings::ResourceType_TimeResource       as isize,
    Width      = bindings::ResourceType_WidthResource      as isize,
    ListLength = bindings::ResourceType_ListLengthResource as isize,
}

impl From<ResourceType> for bindings::ResourceType {
    fn from(value: ResourceType) -> Self {
        return value as bindings::ResourceType;
    }
}
