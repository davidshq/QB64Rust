//! The framing of the Debug Adapter Protocol: a header `Content-Length: <n>`, an empty line (`\r\n\r\n` ends the
//! header), then `n` bytes of JSON. Other header fields are allowed and ignored, as the protocol says.

use serde_json::Value;
use std::io::{BufRead, Write};

/// Reads one message. `None` at the end of the stream before a header begins; an error for a header without a
/// length, a stream that ends inside a message, or a body that is not JSON.
pub fn read_message(r: &mut impl BufRead) -> Result<Option<Value>, String> {
    let mut length = None;
    let mut started = false;
    loop {
        let mut line = Vec::new();
        let n = r.read_until(b'\n', &mut line).map_err(|e| e.to_string())?;
        if n == 0 {
            return if started {
                Err("the stream ended inside a message header".into())
            } else {
                Ok(None)
            };
        }
        started = true;
        let line = line.strip_suffix(b"\n").unwrap_or(&line);
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        if line.is_empty() {
            break;
        }
        if let Some(colon) = line.iter().position(|&b| b == b':') {
            let (name, value) = (&line[..colon], &line[colon + 1..]);
            if name.eq_ignore_ascii_case(b"Content-Length") {
                let digits = value.trim_ascii();
                let parsed = digits
                    .iter()
                    .try_fold(None, |n: Option<usize>, &d| {
                        d.is_ascii_digit()
                            .then(|| n.unwrap_or(0).checked_mul(10)?.checked_add(usize::from(d - b'0')))
                            .flatten()
                            .map(Some)
                    })
                    .flatten()
                    .ok_or_else(|| format!("bad Content-Length `{}`", qb64rust_base::show_bytes(digits)))?;
                length = Some(parsed);
            }
        }
    }
    let length = length.ok_or("a message header without Content-Length")?;
    let mut body = vec![0; length];
    r.read_exact(&mut body)
        .map_err(|e| format!("the stream ended inside a message body: {e}"))?;
    serde_json::from_slice(&body)
        .map(Some)
        .map_err(|e| format!("a message body that is not JSON: {e}"))
}

/// Writes one message and flushes it.
pub fn write_message(w: &mut impl Write, message: &Value) -> Result<(), String> {
    let body = serde_json::to_vec(message).map_err(|e| e.to_string())?;
    let mut out = format!("Content-Length: {}\r\n\r\n", body.len()).into_bytes();
    out.extend_from_slice(&body);
    w.write_all(&out).and_then(|()| w.flush()).map_err(|e| e.to_string())
}

const BASE64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// The base64 of `readMemory` and `writeMemory`.
pub fn base64_encode(bytes: &[u8]) -> String {
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, &b)| n | (u32::from(b) << (16 - 8 * i)));
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(char::from(BASE64[((n >> (18 - 6 * i)) & 63) as usize]));
            } else {
                out.push('=');
            }
        }
    }
    out
}

pub fn base64_decode(text: &str) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let (mut n, mut bits) = (0u32, 0);
    for c in text.bytes().filter(|&c| c != b'=' && !c.is_ascii_whitespace()) {
        let v = BASE64
            .iter()
            .position(|&b| b == c)
            .ok_or_else(|| format!("`{}` is not base64", char::from(c)))?;
        n = (n << 6) | u32::try_from(v).unwrap_or(0);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(u8::try_from((n >> bits) & 0xFF).unwrap_or(0));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn base64() {
        for bytes in [
            &b""[..],
            b"f",
            b"fo",
            b"foo",
            b"foob",
            &[7, 0, 0, 0],
            &[0xFF, 0xFE, 0x80],
        ] {
            assert_eq!(base64_decode(&base64_encode(bytes)).unwrap(), bytes);
        }
        assert_eq!(base64_encode(&[7, 0, 0, 0]), "BwAAAA==");
        assert_eq!(base64_decode("KgAAAA==").unwrap(), [42, 0, 0, 0]);
    }
    use std::io::{BufReader, Read};

    fn framed(v: &Value) -> Vec<u8> {
        let mut out = Vec::new();
        write_message(&mut out, v).unwrap();
        out
    }

    #[test]
    fn one_message() {
        let v = json!({"seq": 1, "type": "request", "command": "initialize"});
        let bytes = framed(&v);
        assert!(bytes.starts_with(b"Content-Length: "));
        let mut r = &bytes[..];
        assert_eq!(read_message(&mut r).unwrap(), Some(v));
        assert_eq!(read_message(&mut r).unwrap(), None);
    }

    #[test]
    fn two_in_one_buffer() {
        let (a, b) = (json!({"seq": 1}), json!({"seq": 2, "body": {"x": "é"}}));
        let mut bytes = framed(&a);
        bytes.extend(framed(&b));
        let mut r = &bytes[..];
        assert_eq!(read_message(&mut r).unwrap(), Some(a));
        assert_eq!(read_message(&mut r).unwrap(), Some(b));
        assert_eq!(read_message(&mut r).unwrap(), None);
    }

    /// A reader that gives one byte per read, as a pipe may.
    struct Trickle<'a>(&'a [u8]);

    impl Read for Trickle<'_> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            match (self.0.split_first(), buf.first_mut()) {
                (Some((&b, rest)), Some(slot)) => {
                    *slot = b;
                    self.0 = rest;
                    Ok(1)
                }
                _ => Ok(0),
            }
        }
    }

    #[test]
    fn a_body_split_over_reads() {
        let v = json!({"seq": 3, "type": "event", "event": "output", "body": {"output": "hello\n"}});
        let bytes = framed(&v);
        let mut r = BufReader::with_capacity(1, Trickle(&bytes));
        assert_eq!(read_message(&mut r).unwrap(), Some(v));
    }

    #[test]
    fn other_header_fields_and_case() {
        let mut r = &b"content-length: 2\r\nContent-Type: application/json\r\n\r\n{}"[..];
        assert_eq!(read_message(&mut r).unwrap(), Some(json!({})));
    }

    #[test]
    fn a_bad_header() {
        let mut r = &b"Content-Length: two\r\n\r\n{}"[..];
        assert!(read_message(&mut r).unwrap_err().contains("bad Content-Length `two`"));
        let mut r = &b"Content-Type: x\r\n\r\n{}"[..];
        assert!(read_message(&mut r).unwrap_err().contains("without Content-Length"));
        let mut r = &b"Content-Length: 10\r\n\r\n{}"[..];
        assert!(read_message(&mut r).unwrap_err().contains("inside a message body"));
        let mut r = &b"Content-Length: 2"[..];
        assert!(read_message(&mut r).unwrap_err().contains("inside a message header"));
    }
}
