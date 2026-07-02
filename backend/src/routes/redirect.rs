//! Validate user-supplied redirect targets.
//!
//! `Redirect::temporary` does **not** normalize upper-case percent-encodes:
//! a `Location: /%2Fevil` header gets handed to the browser verbatim. So we
//! can't trust naive byte matching alone — we do an explicit, case-insensitive
//! scan for the dangerous byte sequences and also reject classic pitfalls
//! like scheme-relative URLs and back-slashes.
//!
//! ## Rejected inputs (non-exhaustive)
//!
//! - empty string
//! - anything that doesn't start with `/`
//! - `//host` (scheme-relative; would redirect off-site)
//! - any literal `\` (defence against browsers that treat `\` like `/`)
//! - any case of `%2f` (forward slash) or `%5c` (backslash), including
//!   double-encoded `%252f`/`%255c`
//! - control characters and whitespace

/// Strict-but-permissive-enough validator for same-origin redirect targets.
///
/// Returns `true` only for paths that begin with a single `/`, contain no
/// scheme or authority, and don't try to smuggle `/` or `\` via percent
/// encoding (single or double).
#[must_use]
pub fn is_valid_redirect_url(url: &str) -> bool {
    if url.is_empty() || !url.starts_with('/') {
        return false;
    }
    // Scheme-relative: `//host/path` — `Redirect::temporary` would treat it as
    // off-site. `///host` is also off-site for most browsers.
    if url.starts_with("//") || url.starts_with('/') && url[1..].starts_with('/') {
        return false;
    }
    // Reject literal backslashes outright. Some browsers normalise `\` to `/`
    // in the URL bar, which would let `/\example.com` become off-site.
    if url.contains('\\') {
        return false;
    }
    if contains_control_or_whitespace(url) {
        return false;
    }

    // Scan byte-by-byte for any percent encoding that decodes (or could
    // decode after another round of decoding) to `/` or `\`.
    scan_percent_smuggle(url)
}

fn contains_control_or_whitespace(url: &str) -> bool {
    url.chars().any(|c| c.is_control() || c.is_whitespace())
}

/// Walk the byte stream looking for percent-encoded `/`, `\`, or double-encoded
/// variants thereof.
///
/// The rules, case-insensitive:
/// - `%2f`, `%2F`            → `/` (single decode)
/// - `%5c`, `%5C`            → `\` (single decode)
/// - `%252f`, `%252F`        → `%2f` after one decode → `/` after second
/// - `%255c`, `%255C`        → `%5c` after one decode → `\` after second
fn scan_percent_smuggle(url: &str) -> bool {
    let bytes = url.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        // Case-insensitive match on `%`, but operate on bytes so the
        // detector works for any locale.
        if b == b'%' && i + 2 < bytes.len() {
            let hex1 = bytes[i + 1].to_ascii_lowercase();
            let hex2 = bytes[i + 2].to_ascii_lowercase();
            // Single decode: %2f or %5c.
            if (hex1 == b'2' && hex2 == b'f') || (hex1 == b'5' && hex2 == b'c') {
                return false;
            }
            // Double decode: %252f, %255c.
            if i + 4 < bytes.len() && bytes[i] == b'%' {
                let hex3 = bytes[i + 3].to_ascii_lowercase();
                let hex4 = bytes[i + 4].to_ascii_lowercase();
                if (hex1 == b'2' && hex2 == b'5' && hex3 == b'2' && hex4 == b'f')
                    || (hex1 == b'2' && hex2 == b'5' && hex3 == b'5' && hex4 == b'c')
                {
                    return false;
                }
            }
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_normal_paths() {
        for url in [
            "/",
            "/login",
            "/path/to/thing",
            "/items/123",
            "/with%20space",
            "/with-dash_and.dot",
        ] {
            assert!(is_valid_redirect_url(url), "should accept {url}");
        }
    }

    #[test]
    fn rejects_empty() {
        assert!(!is_valid_redirect_url(""));
    }

    #[test]
    fn rejects_non_leading_slash() {
        for url in ["login", "http://evil.com/", "about:blank", "?q=1"] {
            assert!(!is_valid_redirect_url(url), "should reject {url}");
        }
    }

    #[test]
    fn rejects_scheme_relative() {
        for url in ["//evil.com", "//evil.com/path", "///evil.com"] {
            assert!(!is_valid_redirect_url(url), "should reject {url}");
        }
    }

    #[test]
    fn rejects_backslash() {
        for url in ["/\\evil", "/\\\\foo", r#"/path\..\etc"#] {
            assert!(!is_valid_redirect_url(url), "should reject {url}");
        }
    }

    #[test]
    fn rejects_percent_encoded_slash_uppercase() {
        assert!(!is_valid_redirect_url("/%2Fevil"));
        assert!(!is_valid_redirect_url("/path%2fother"));
    }

    #[test]
    fn rejects_percent_encoded_backslash() {
        assert!(!is_valid_redirect_url("/%5Cevil"));
        assert!(!is_valid_redirect_url("/%5cother"));
    }

    #[test]
    fn rejects_double_encoded_slash() {
        assert!(!is_valid_redirect_url("/%252fevil"));
        assert!(!is_valid_redirect_url("/%252Fevil"));
    }

    #[test]
    fn rejects_double_encoded_backslash() {
        assert!(!is_valid_redirect_url("/%255cevil"));
        assert!(!is_valid_redirect_url("/%255Cevil"));
    }

    #[test]
    fn rejects_control_characters() {
        for url in ["/path\nwith-newline", "/path\twith-tab", "/with\x00null"] {
            assert!(!is_valid_redirect_url(url), "should reject {url:?}");
        }
    }

    #[test]
    fn rejects_spaces_anywhere() {
        for url in ["/path with-space", "/ leading-space", "/trailing "] {
            assert!(!is_valid_redirect_url(url), "should reject {url:?}");
        }
    }

    #[test]
    fn rejects_protocol_relative_with_encoded_slash() {
        // `//foo` already rejected by the scheme-relative check.
        assert!(!is_valid_redirect_url("/%2F%2Fevil"));
    }
}
