use axum::http::{HeaderMap, HeaderValue, header::ACCEPT_ENCODING};

/// Check if the client supports the given encoding.
pub(crate) fn supports_encoding(headers: &HeaderMap, encoding: &str) -> bool {
    let Some(header_value) = headers
        .get(ACCEPT_ENCODING)
        .and_then(|v: &HeaderValue| v.to_str().ok())
    else {
        return false;
    };

    header_value
        .split_whitespace()
        .collect::<String>()
        .split(',')
        .filter_map(|item| {
            let mut parts = item.splitn(2, ";q=");
            let encoding = parts.next();

            // Any q-value parsing to 0.0 (e.g. `0`, `0.0`, `0.000`) signals
            // that the client does not accept the encoding.
            let rejected = parts
                .next()
                .and_then(|q| q.parse::<f32>().ok())
                .is_some_and(|q| q <= 0.0);

            if rejected { None } else { encoding }
        })
        .any(|v| v == encoding || v == "*")
}

#[cfg(test)]
mod tests {
    use super::supports_encoding;
    use axum::http::{HeaderMap, HeaderValue, header::ACCEPT_ENCODING};

    fn check(header: &str, encoding: &str) -> bool {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_str(header).unwrap());

        supports_encoding(&headers, encoding)
    }

    #[test]
    fn accept_encoding() {
        assert!(check("gzip", "gzip"));
        assert!(check("gzip, compress, br", "gzip"));
        assert!(check("br;q=1.0, gzip;q=0.8, *;q=0.1", "gzip"));
        assert!(!check("gzip", "br"));
        assert!(check("gzip, compress, br", "br"));
        assert!(check("br;q=1.0, gzip;q=0.8, *;q=0.1", "br"));
        assert!(!check("gzip", "compress"));
        assert!(check("gzip, compress, br", "compress"));
        assert!(check("br;q=1.0, gzip;q=0.8, *;q=0.1", "compress"));
        assert!(!check("gzip", "zstd"));
        assert!(!check("gzip, compress, br", "zstd"));
        assert!(check("br;q=1.0, gzip;q=0.8, *;q=0.1", "zstd"));

        // q=0 in any spelling means the client refuses that encoding.
        assert!(!check("gzip;q=0", "gzip"));
        assert!(!check("gzip;q=0.0", "gzip"));
        assert!(!check("gzip;q=0.000", "gzip"));
        assert!(!check("br;q=1.0, gzip;q=0.0", "gzip"));
        assert!(!check("*;q=0", "gzip"));
        assert!(!check("*;q=0.0", "gzip"));
    }
}
