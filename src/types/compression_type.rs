use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CompressionType {
    Undefined = bindings::CompressionType_UndefinedCompression,
    B44A = bindings::CompressionType_B44ACompression,
    B44 = bindings::CompressionType_B44Compression,
    BZip = bindings::CompressionType_BZipCompression,
    DXT1 = bindings::CompressionType_DXT1Compression,
    DXT3 = bindings::CompressionType_DXT3Compression,
    DXT5 = bindings::CompressionType_DXT5Compression,
    Fax = bindings::CompressionType_FaxCompression,
    Group4 = bindings::CompressionType_Group4Compression,
    JBIG1 = bindings::CompressionType_JBIG1Compression,
    JBIG2 = bindings::CompressionType_JBIG2Compression,
    JPEG2000 = bindings::CompressionType_JPEG2000Compression,
    JPEG = bindings::CompressionType_JPEGCompression,
    LosslessJPEG = bindings::CompressionType_LosslessJPEGCompression,
    LZMA = bindings::CompressionType_LZMACompression,
    LZW = bindings::CompressionType_LZWCompression,
    No = bindings::CompressionType_NoCompression,
    Piz = bindings::CompressionType_PizCompression,
    Pxr24 = bindings::CompressionType_Pxr24Compression,
    RLE = bindings::CompressionType_RLECompression,
    Zip = bindings::CompressionType_ZipCompression,
    ZipS = bindings::CompressionType_ZipSCompression,
    Zstd = bindings::CompressionType_ZstdCompression,
    WebP = bindings::CompressionType_WebPCompression,
    DWAA = bindings::CompressionType_DWAACompression,
    DWAB = bindings::CompressionType_DWABCompression,
    BC7 = bindings::CompressionType_BC7Compression,
    BC5 = bindings::CompressionType_BC5Compression,
    LERC = bindings::CompressionType_LERCCompression,
}

impl Default for CompressionType {
    fn default() -> Self {
        return CompressionType::Undefined;
    }
}

impl From<CompressionType> for bindings::CompressionType {
    fn from(value: CompressionType) -> Self {
        return value as bindings::CompressionType;
    }
}

impl From<bindings::CompressionType> for CompressionType {
    fn from(value: bindings::CompressionType) -> Self {
        /*
         * SAFETY:
         *
         * `CompressionType` has the same repr as `bindings::CompressionType` - u32
         *
         * If `value` is less than LERC than it is in the vaild range and can be safely
         * reinterpreted as `CompressionType`
         */
        if value <= bindings::CompressionType_LERCCompression {
            return unsafe { std::mem::transmute(value) };
        }
        return CompressionType::default();
    }
}
