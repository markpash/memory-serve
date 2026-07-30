use std::io::{Read, Write};

/// Decompress a byte slice using brotli.
#[cfg(feature = "brotli")]
pub(crate) fn decompress_brotli(input: &[u8]) -> Option<Vec<u8>> {
    let mut writer = brotli::DecompressorWriter::new(Vec::new(), 1024);
    writer.write_all(input).ok()?;

    writer.into_inner().ok()
}

/// Compress a byte slice using brotli.
#[cfg(feature = "brotli")]
pub(crate) fn compress_brotli(input: &[u8]) -> Option<Vec<u8>> {
    let mut writer = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
    writer.write_all(input).ok()?;

    Some(writer.into_inner())
}

/// Compress a byte slice using gzip.
pub(crate) fn compress_gzip(input: &[u8]) -> Option<Vec<u8>> {
    let mut writer = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
    writer.write_all(input).ok()?;

    writer.finish().ok()
}

/// Decompress a byte slice using gzip.
pub(crate) fn decompress_gzip(input: &[u8]) -> Option<Vec<u8>> {
    let mut decoder = flate2::read::GzDecoder::new(input);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out).ok()?;

    Some(out)
}

/// Compress a byte slice using the codec embedded assets are stored with:
/// brotli when the `brotli` feature is enabled, gzip otherwise.
pub(crate) fn compress_embed(input: &[u8]) -> Option<Vec<u8>> {
    #[cfg(feature = "brotli")]
    {
        compress_brotli(input)
    }
    #[cfg(not(feature = "brotli"))]
    {
        compress_gzip(input)
    }
}

#[cfg(test)]
mod tests {
    use super::{compress_gzip, decompress_gzip};

    /// Repetitive, compressible sample payload.
    const SAMPLE: &[u8] = b"<html><body>hello hello hello world world world</body></html>";

    #[cfg(feature = "brotli")]
    #[test]
    fn brotli_roundtrip() {
        use super::{compress_brotli, decompress_brotli};

        let compressed = compress_brotli(SAMPLE).expect("compresses");
        // Repetitive input should actually shrink.
        assert!(compressed.len() < SAMPLE.len());
        assert_eq!(decompress_brotli(&compressed).as_deref(), Some(SAMPLE));

        let empty = compress_brotli(b"").expect("compresses");
        assert_eq!(decompress_brotli(&empty).as_deref(), Some(&b""[..]));
    }

    #[test]
    fn gzip_roundtrip() {
        let compressed = compress_gzip(SAMPLE).expect("compresses");
        assert_eq!(decompress_gzip(&compressed).as_deref(), Some(SAMPLE));

        let empty = compress_gzip(b"").expect("compresses");
        assert_eq!(decompress_gzip(&empty).as_deref(), Some(&b""[..]));
    }

    #[cfg(feature = "brotli")]
    #[test]
    fn decompress_brotli_rejects_garbage() {
        // Arbitrary bytes are not a valid brotli stream.
        assert_eq!(
            super::decompress_brotli(&[0xff, 0xff, 0xff, 0xff]),
            None::<Vec<u8>>
        );
    }

    #[test]
    fn decompress_gzip_rejects_garbage() {
        // Arbitrary bytes are not a valid gzip stream.
        assert_eq!(decompress_gzip(&[0xff, 0xff, 0xff, 0xff]), None::<Vec<u8>>);
    }
}
