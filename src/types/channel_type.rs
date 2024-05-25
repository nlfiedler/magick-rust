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

#[derive(Debug, Clone, Copy)]
pub enum ChannelType {
    Undefined,
    RedChannel,
    GrayChannel,
    CyanChannel,
    LChannel,
    GreenChannel,
    MagentaChannel,
    aChannel,
    BlueChannel,
    bChannel,
    YellowChannel,
    BlackChannel,
    AlphaChannel,
    OpacityChannel,
    IndexChannel,
    ReadMaskChannel,
    WriteMaskChannel,
    MetaChannel,
    CompositeMaskChannel,
    CompositeChannels,
    AllChannels,
    TrueAlphaChannel,
    RGBChannels,
    GrayChannels,
    SyncChannels,
    DefaultChannels,
}

impl Default for ChannelType {
    fn default() -> Self {
        return ChannelType::DefaultChannels;
    }
}

impl From<ChannelType> for bindings::ChannelType {
    fn from(value: ChannelType) -> Self {
        match value {
            ChannelType::Undefined => bindings::ChannelType_UndefinedChannel,
            ChannelType::RedChannel => bindings::ChannelType_RedChannel,
            ChannelType::GrayChannel => bindings::ChannelType_GrayChannel,
            ChannelType::CyanChannel => bindings::ChannelType_CyanChannel,
            ChannelType::LChannel => bindings::ChannelType_LChannel,
            ChannelType::GreenChannel => bindings::ChannelType_GreenChannel,
            ChannelType::MagentaChannel => bindings::ChannelType_MagentaChannel,
            ChannelType::aChannel => bindings::ChannelType_aChannel,
            ChannelType::BlueChannel => bindings::ChannelType_BlueChannel,
            ChannelType::bChannel => bindings::ChannelType_bChannel,
            ChannelType::YellowChannel => bindings::ChannelType_YellowChannel,
            ChannelType::BlackChannel => bindings::ChannelType_BlackChannel,
            ChannelType::AlphaChannel => bindings::ChannelType_AlphaChannel,
            ChannelType::OpacityChannel => bindings::ChannelType_OpacityChannel,
            ChannelType::IndexChannel => bindings::ChannelType_IndexChannel,
            ChannelType::ReadMaskChannel => bindings::ChannelType_ReadMaskChannel,
            ChannelType::WriteMaskChannel => bindings::ChannelType_WriteMaskChannel,
            ChannelType::MetaChannel => bindings::ChannelType_MetaChannel,
            ChannelType::CompositeMaskChannel => bindings::ChannelType_CompositeMaskChannel,
            ChannelType::CompositeChannels => bindings::ChannelType_CompositeChannels,
            ChannelType::AllChannels => bindings::ChannelType_AllChannels,
            ChannelType::TrueAlphaChannel => bindings::ChannelType_TrueAlphaChannel,
            ChannelType::RGBChannels => bindings::ChannelType_RGBChannels,
            ChannelType::GrayChannels => bindings::ChannelType_GrayChannels,
            ChannelType::SyncChannels => bindings::ChannelType_SyncChannels,
            ChannelType::DefaultChannels => bindings::ChannelType_DefaultChannels,
        }
    }
}

impl From<bindings::ChannelType> for ChannelType {
    fn from(value: bindings::ChannelType) -> Self {
        // Unreachable match arms commented out
        match value {
            bindings::ChannelType_UndefinedChannel => ChannelType::Undefined,
            bindings::ChannelType_RedChannel => ChannelType::RedChannel,
            // bindings::ChannelType_GrayChannel          => { ChannelType::GrayChannel          },
            // bindings::ChannelType_CyanChannel          => { ChannelType::CyanChannel          },
            // bindings::ChannelType_LChannel             => { ChannelType::LChannel             },
            bindings::ChannelType_GreenChannel => ChannelType::GreenChannel,
            // bindings::ChannelType_MagentaChannel       => { ChannelType::MagentaChannel       },
            // bindings::ChannelType_aChannel             => { ChannelType::aChannel             },
            bindings::ChannelType_BlueChannel => ChannelType::BlueChannel,
            // bindings::ChannelType_bChannel             => { ChannelType::bChannel             },
            // bindings::ChannelType_YellowChannel        => { ChannelType::YellowChannel        },
            bindings::ChannelType_BlackChannel => ChannelType::BlackChannel,
            bindings::ChannelType_AlphaChannel => ChannelType::AlphaChannel,
            // bindings::ChannelType_OpacityChannel       => { ChannelType::OpacityChannel       },
            bindings::ChannelType_IndexChannel => ChannelType::IndexChannel,
            bindings::ChannelType_ReadMaskChannel => ChannelType::ReadMaskChannel,
            bindings::ChannelType_WriteMaskChannel => ChannelType::WriteMaskChannel,
            bindings::ChannelType_MetaChannel => ChannelType::MetaChannel,
            bindings::ChannelType_CompositeMaskChannel => ChannelType::CompositeMaskChannel,
            bindings::ChannelType_CompositeChannels => ChannelType::CompositeChannels,
            bindings::ChannelType_AllChannels => ChannelType::AllChannels,
            // bindings::ChannelType_TrueAlphaChannel     => { ChannelType::TrueAlphaChannel     },
            // bindings::ChannelType_RGBChannels          => { ChannelType::RGBChannels          },
            bindings::ChannelType_GrayChannels => ChannelType::GrayChannels,
            bindings::ChannelType_SyncChannels => ChannelType::SyncChannels,
            // bindings::ChannelType_DefaultChannels      => { ChannelType::DefaultChannels      },
            _ => ChannelType::Undefined,
        }
    }
}
