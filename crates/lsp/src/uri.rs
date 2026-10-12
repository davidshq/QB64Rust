//! `file:` URIs and paths, and the key that identifies a file whether it is open in the editor or on disk.

use lsp_types::Uri;
use std::path::{Component, Path, PathBuf};
use std::str::FromStr as _;

/// The path of a `file:` URI; `None` for other schemes (`untitled:`) and malformed ones.
pub fn to_path(uri: &Uri) -> Option<PathBuf> {
    let rest = uri.as_str().strip_prefix("file://")?;
    let (authority, path) = rest.split_at(rest.find('/')?);
    let path = percent_decode(path)?;
    let path = if cfg!(windows) {
        let p = path.replace('/', "\\");
        if !authority.is_empty() {
            // A UNC path: file://server/share/x.bas
            format!("\\\\{}{p}", percent_decode(authority)?)
        } else if p.len() >= 3 && p.as_bytes()[2] == b':' && p.as_bytes()[1].is_ascii_alphabetic() {
            // `\c:\x.bas`: the drive letter after the leading separator.
            p[1..].to_string()
        } else {
            p
        }
    } else if authority.is_empty() {
        path
    } else {
        return None;
    };
    Some(PathBuf::from(path))
}

/// The `file:` URI of a path, as VS Code writes it: `/` separators, the drive letter lower case and its `:` as
/// `%3A`, and every byte outside `A-Z a-z 0-9 - . _ ~ /` percent-encoded.
pub fn from_path(path: &Path) -> Uri {
    let s = path.to_string_lossy().replace('\\', "/");
    let (authority, s) = match s.strip_prefix("//") {
        Some(unc) => match unc.find('/') {
            Some(i) => (unc[..i].to_string(), unc[i..].to_string()),
            None => (unc.to_string(), "/".to_string()),
        },
        None => (String::new(), s),
    };
    let mut out = format!("file://{}", percent_encode(&authority));
    let bytes = s.as_bytes();
    let rest = if bytes.len() >= 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
        out.push('/');
        out.push(char::from(bytes[0].to_ascii_lowercase()));
        out.push_str("%3A");
        &s[2..]
    } else {
        if !s.starts_with('/') {
            out.push('/');
        }
        &s[..]
    };
    out.push_str(&percent_encode(rest));
    Uri::from_str(&out).expect("a percent-encoded file URI is valid")
}

/// Identifies a file: its absolute path with `.` and `..` resolved, `/` separators, lower case on Windows (where
/// names are case-insensitive). Not canonical: links are not followed, so a file reached through a link and
/// directly gives two keys (and is parsed as two files, which only `$INCLUDEONCE` could tell).
pub fn path_key(path: &Path) -> String {
    let abs = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    let mut parts: Vec<Component> = Vec::new();
    for c in abs.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                if matches!(parts.last(), Some(Component::Normal(_))) {
                    parts.pop();
                }
            }
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => parts.push(c),
        }
    }
    let norm: PathBuf = parts.iter().collect();
    let key = norm.to_string_lossy().replace('\\', "/");
    if cfg!(windows) { key.to_lowercase() } else { key }
}

/// The key of a document: its path's for a `file:` URI, the URI itself otherwise.
pub fn uri_key(uri: &Uri) -> String {
    match to_path(uri) {
        Some(p) => path_key(&p),
        None => uri.as_str().to_string(),
    }
}

fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~' | b'/') {
            out.push(char::from(b));
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// `None` when the result is not UTF-8 (a URI from VS Code always is).
fn percent_decode(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = |b: u8| char::from(b).to_digit(16);
            if let (Some(h), Some(l)) = (hex(bytes[i + 1]), hex(bytes[i + 2])) {
                out.push(u8::try_from(h * 16 + l).expect("two hex digits"));
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    #[expect(clippy::disallowed_methods, reason = "a file name from a URI, not BASIC source")]
    String::from_utf8(out).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uri(s: &str) -> Uri {
        Uri::from_str(s).unwrap()
    }

    // Made-up paths: `does\not\exist` and `does/not/exist` are what `tools\repo_check` accepts as such (rule 2).
    #[cfg(windows)]
    #[test]
    fn windows_paths() {
        let u = uri("file:///c%3A/does/not/exist/My%20Code/x%23.bas");
        let p = "c:\\does\\not\\exist\\My Code\\x#.bas";
        assert_eq!(to_path(&u), Some(PathBuf::from(p)));
        let upper = "C:\\does\\not\\exist\\My Code\\x#.bas";
        assert_eq!(from_path(Path::new(upper)).as_str(), u.as_str());
        let slashes = "file:///C:/does/not/exist/b.bas";
        assert_eq!(
            to_path(&uri(slashes)),
            Some(PathBuf::from("C:\\does\\not\\exist\\b.bas"))
        );
        let unc = uri("file://server/share/a.bas");
        assert_eq!(to_path(&unc), Some(PathBuf::from("\\\\server\\share\\a.bas")));
        assert_eq!(
            from_path(Path::new("\\\\server\\share\\a.bas")).as_str(),
            "file://server/share/a.bas"
        );
        let dots = "C:\\does\\not\\exist\\.\\b\\..\\X.BAS";
        assert_eq!(path_key(Path::new(dots)), "c:/does/not/exist/x.bas");
        assert_eq!(uri_key(&u), "c:/does/not/exist/my code/x#.bas");
    }

    #[cfg(not(windows))]
    #[test]
    fn unix_paths() {
        let u = uri("file:///does/not/exist/My%20Code/x.bas");
        assert_eq!(to_path(&u), Some(PathBuf::from("/does/not/exist/My Code/x.bas")));
        assert_eq!(
            from_path(Path::new("/does/not/exist/My Code/x.bas")).as_str(),
            u.as_str()
        );
        assert_eq!(path_key(Path::new("/a/./b/../X.bas")), "/a/X.bas");
    }

    #[test]
    fn other_schemes() {
        let u = uri("untitled:Untitled-1");
        assert_eq!(to_path(&u), None);
        assert_eq!(uri_key(&u), "untitled:Untitled-1");
    }

    #[test]
    fn non_ascii_round_trip() {
        let p = std::env::temp_dir().join("\u{e9}t\u{e9}.bas");
        let u = from_path(&p);
        assert!(u.as_str().contains("%C3%A9t%C3%A9.bas"));
        assert_eq!(path_key(&to_path(&u).unwrap()), path_key(&p));
    }
}
