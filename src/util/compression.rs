use std::io::Write;

/// Decompress a byte slice using brotli.
pub(crate) fn decompress_brotli(input: &[u8]) -> Option<Vec<u8>> {
    let mut writer = brotli::DecompressorWriter::new(Vec::new(), 1024);
    writer.write_all(input).ok()?;

    writer.into_inner().ok()
}

/// Compress a byte slice using brotli.
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

#[cfg(test)]
mod tests {
    use super::{compress_brotli, compress_gzip, decompress_brotli};
    use std::io::Read;

    /// Repetitive, compressible sample payload.
    const SAMPLE: &[u8] = b"<html><body>hello hello hello world world world</body></html>";

    fn gunzip(input: &[u8]) -> Vec<u8> {
        let mut decoder = flate2::read::GzDecoder::new(input);
        let mut out = Vec::new();
        decoder.read_to_end(&mut out).expect("valid gzip stream");
        out
    }

    #[test]
    fn brotli_roundtrip() {
        let compressed = compress_brotli(SAMPLE).expect("compresses");
        // Repetitive input should actually shrink.
        assert!(compressed.len() < SAMPLE.len());
        assert_eq!(decompress_brotli(&compressed).as_deref(), Some(SAMPLE));
    }

    #[test]
    fn gzip_roundtrip() {
        let compressed = compress_gzip(SAMPLE).expect("compresses");
        assert_eq!(gunzip(&compressed), SAMPLE);
    }

    #[test]
    fn empty_input_roundtrips() {
        let brotli = compress_brotli(b"").expect("compresses");
        assert_eq!(decompress_brotli(&brotli).as_deref(), Some(&b""[..]));

        let gzip = compress_gzip(b"").expect("compresses");
        assert_eq!(gunzip(&gzip), b"");
    }

    #[test]
    fn decompress_brotli_rejects_garbage() {
        // Arbitrary bytes are not a valid brotli stream.
        assert_eq!(decompress_brotli(&[0xff, 0xff, 0xff, 0xff]), None);
    }
}
