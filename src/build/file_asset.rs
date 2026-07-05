use std::path::PathBuf;

/// Internal data structure
pub(super) struct FileAsset {
    pub(super) route: String,
    pub(super) path: PathBuf,
    pub(super) etag: String,
    pub(super) content_type: String,
    pub(super) compressed_bytes: Option<Vec<u8>>,
    pub(super) should_compress: bool,
}

impl PartialEq for FileAsset {
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route
    }
}

impl Eq for FileAsset {}

impl PartialOrd for FileAsset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileAsset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.route.cmp(&other.route)
    }
}

#[cfg(test)]
mod tests {
    use super::FileAsset;
    use std::path::PathBuf;

    /// Build a `FileAsset` that only differs from others by its route; the
    /// remaining fields are irrelevant to equality and ordering.
    fn asset(route: &str) -> FileAsset {
        FileAsset {
            route: route.to_string(),
            path: PathBuf::from("/tmp/whatever"),
            etag: "etag".to_string(),
            content_type: "text/plain".to_string(),
            compressed_bytes: None,
            should_compress: false,
        }
    }

    #[test]
    fn equality_is_route_only() {
        let mut a = asset("/index.html");
        let b = asset("/index.html");
        // Differ in every field except the route.
        a.etag = "different".to_string();
        a.content_type = "text/html".to_string();
        a.should_compress = true;

        assert!(a == b);
        assert!(asset("/a.html") != asset("/b.html"));
    }

    #[test]
    fn ordering_and_sort_are_by_route() {
        assert!(asset("/a.html") < asset("/b.html"));

        let mut assets = [asset("/c"), asset("/a"), asset("/b")];
        assets.sort();
        let routes: Vec<&str> = assets.iter().map(|a| a.route.as_str()).collect();
        assert_eq!(routes, ["/a", "/b", "/c"]);
    }
}
