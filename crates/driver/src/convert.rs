//! `qb64rust convert --to utf8|cp437 <files>` (OpenSpec change `utf8-source`, design D9; spec
//! `compiler/source-encoding`, "The convert command"): rewrites whole source files from code page 437 to UTF-8 or
//! back, in place. It knows nothing of BASIC: a comment is converted like a string, and an `'$ENCODING` line is
//! neither added nor changed. Every file is read and converted first; none is written unless all can be.

use qb64rust_base::cp437;
use std::path::{Path, PathBuf};

/// The encoding `convert` writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum To {
    Utf8,
    Cp437,
}

impl To {
    pub fn from_name(name: &str) -> Option<To> {
        match name {
            "utf8" => Some(To::Utf8),
            "cp437" => Some(To::Cp437),
            _ => None,
        }
    }
}

/// The 1-based line of the byte at `at`.
fn line_of(bytes: &[u8], at: usize) -> usize {
    1 + bytes[..at].iter().filter(|&&b| b == b'\n').count()
}

/// The bytes of `bytes` in the encoding `to`: `Ok(None)` for a file of bytes below 128 only, which is left as it
/// is; `Err` the line and the reason a file cannot be converted.
pub fn convert(bytes: &[u8], to: To) -> Result<Option<Vec<u8>>, (usize, String)> {
    if bytes.is_ascii() {
        return Ok(None);
    }
    match to {
        To::Utf8 => {
            if cp437::first_bad_utf8(bytes).is_none() {
                let at = bytes.iter().position(|&b| b > 127).unwrap_or(0);
                return Err((line_of(bytes, at), "the file is UTF-8 already".to_string()));
            }
            let mut out = Vec::with_capacity(bytes.len() * 2);
            let mut buf = [0; 4];
            for &b in bytes {
                out.extend_from_slice(cp437::char_of(b).encode_utf8(&mut buf).as_bytes());
            }
            Ok(Some(out))
        }
        To::Cp437 => {
            // A code page 437 file holds no byte order mark.
            let start = if bytes.starts_with(qb64rust_syntax::encoding::BOM) {
                3
            } else {
                0
            };
            let mut out = Vec::with_capacity(bytes.len());
            let mut at = start;
            while at < bytes.len() {
                let Some((ch, len)) = cp437::next_char(&bytes[at..]) else {
                    let reason = format!("the byte 0x{:02X} is not UTF-8", bytes[at]);
                    return Err((line_of(bytes, at), reason));
                };
                let Some(b) = cp437::byte_of(ch) else {
                    let reason = format!("`{ch}` (U+{:04X}) has no code page 437 byte", u32::from(ch));
                    return Err((line_of(bytes, at), reason));
                };
                out.push(b);
                at += len;
            }
            Ok(Some(out))
        }
    }
}

/// What a run did: the files written, and those left as they are (bytes below 128 only).
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Converted {
    pub written: Vec<PathBuf>,
    pub left: Vec<PathBuf>,
}

/// Why a run stopped: one line per file that cannot be read, converted or written, as `<file>:<line>: <reason>`,
/// and the files written before a write failed. When a file cannot be read or converted, none is written.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Failed {
    pub problems: Vec<String>,
    pub written: Vec<PathBuf>,
}

/// Converts `files` in place.
pub fn run(files: &[PathBuf], to: To) -> Result<Converted, Failed> {
    let mut problems = Vec::new();
    let mut plans: Vec<(&Path, Option<Vec<u8>>)> = Vec::new();
    for file in files {
        match std::fs::read(file) {
            Err(e) => problems.push(format!("{}: cannot be read: {e}", file.display())),
            Ok(bytes) => match convert(&bytes, to) {
                Ok(out) => plans.push((file, out)),
                Err((line, reason)) => problems.push(format!("{}:{line}: {reason}", file.display())),
            },
        }
    }
    if !problems.is_empty() {
        return Err(Failed {
            problems,
            written: Vec::new(),
        });
    }
    let mut done = Converted::default();
    for (file, out) in plans {
        match out {
            Some(bytes) => {
                // A write can still fail (a read-only or locked file); the files before it are then converted.
                if let Err(e) = std::fs::write(file, bytes) {
                    return Err(Failed {
                        problems: vec![format!("{}: cannot be written: {e}", file.display())],
                        written: done.written,
                    });
                }
                done.written.push(file.to_path_buf());
            }
            None => done.left.push(file.to_path_buf()),
        }
    }
    Ok(done)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_games_file() {
        let out = convert(b"PRINT \"\xC4\"\r\n", To::Utf8).unwrap().unwrap();
        assert_eq!(out, b"PRINT \"\xE2\x94\x80\"\r\n");
    }

    #[test]
    fn there_and_back() {
        let all: Vec<u8> = (0..=255).collect();
        let utf8 = convert(&all, To::Utf8).unwrap().unwrap();
        assert_eq!(convert(&utf8, To::Cp437).unwrap().unwrap(), all);
    }

    #[test]
    fn ascii_is_left_alone() {
        assert_eq!(convert(b"PRINT 1\n", To::Utf8), Ok(None));
        assert_eq!(convert(b"PRINT 1\n", To::Cp437), Ok(None));
    }

    #[test]
    fn what_cannot_be_converted() {
        let utf8 = "' a\nPRINT \"\u{e9}\"\n".as_bytes();
        assert_eq!(
            convert(utf8, To::Utf8),
            Err((2, "the file is UTF-8 already".to_string()))
        );
        let japanese = "PRINT 1\n' \u{65e5}\n".as_bytes();
        assert_eq!(
            convert(japanese, To::Cp437),
            Err((2, "`\u{65e5}` (U+65E5) has no code page 437 byte".to_string()))
        );
        assert_eq!(
            convert(b"\n\nPRINT \"\xC4\"\n", To::Cp437),
            Err((3, "the byte 0xC4 is not UTF-8".to_string()))
        );
    }

    /// A file that cannot be written stops the run, and the files written before it are named.
    #[test]
    fn a_write_that_fails_names_what_was_written() {
        let d = tempfile::tempdir().unwrap();
        let a = d.path().join("a.bas");
        let b = d.path().join("b.bas");
        std::fs::write(&a, b"PRINT \"\xC4\"\n").unwrap();
        std::fs::write(&b, b"PRINT \"\xC4\"\n").unwrap();
        let mut read_only = std::fs::metadata(&b).unwrap().permissions();
        read_only.set_readonly(true);
        std::fs::set_permissions(&b, read_only.clone()).unwrap();
        let result = run(&[a.clone(), b.clone()], To::Utf8);
        #[expect(
            clippy::permissions_set_readonly_false,
            reason = "the temporary folder must be removable"
        )]
        read_only.set_readonly(false);
        std::fs::set_permissions(&b, read_only).unwrap();
        // Root on Linux writes a read-only file, so nothing fails there and there is nothing to test.
        let Err(failed) = result else {
            eprintln!("skipped: a read-only file could be written (run as root?)");
            return;
        };
        assert_eq!(failed.written, vec![a.clone()]);
        assert_eq!(failed.problems.len(), 1);
        assert!(failed.problems[0].contains("b.bas: cannot be written"), "{failed:?}");
        assert_eq!(std::fs::read(&a).unwrap(), b"PRINT \"\xE2\x94\x80\"\n");
    }

    #[test]
    fn a_byte_order_mark_is_dropped_for_code_page_437() {
        assert_eq!(
            convert(b"\xEF\xBB\xBFPRINT \"\xC3\xA9\"", To::Cp437),
            Ok(Some(b"PRINT \"\x82\"".to_vec()))
        );
    }
}
