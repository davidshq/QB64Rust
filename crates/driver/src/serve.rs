//! `qb64rust serve`: a web folder served on this machine the way a web host must serve it (spec
//! `compiler/web-serve`; OpenSpec change `wasm-publish`, design D3). A page that runs threads needs two headers and
//! the right content type for its `.wasm` file, and cannot be opened from the disk.
//!
//! The server listens on `127.0.0.1` only, so no other machine reaches it and Windows asks nothing about it. It
//! answers `GET` and `HEAD`, one request a connection, and writes nothing into the folder. What a request gets is
//! decided by [`respond`], from the request's method and target alone, so that it is tested without a socket.

use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpListener};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// The port of a `serve` without `--port`. It is fixed because a browser keeps a page's saved games for its
/// address with the port: another port each run would lose them.
pub const DEFAULT_PORT: u16 = 8064;

/// The last port a `serve` without `--port` tries when [`DEFAULT_PORT`] and those after it are taken.
pub const LAST_PORT: u16 = 8083;

/// The two headers a page needs to run threads, which `--no-headers` leaves out.
pub const ISOLATION_HEADERS: [(&str, &str); 2] = [
    ("Cross-Origin-Opener-Policy", "same-origin"),
    ("Cross-Origin-Embedder-Policy", "require-corp"),
];

/// The longest request head read; a longer one is refused.
const HEAD_LIMIT: usize = 16 * 1024;

/// What a request gets: a file of the folder, or a status with a short text.
#[derive(Debug, PartialEq, Eq)]
pub enum Answer {
    File(PathBuf),
    Status(u16, &'static str),
}

/// The content type of a file by its ending. A browser compiles a `.wasm` file while it loads only when it is
/// `application/wasm`. An ending not listed is `application/octet-stream`.
pub fn content_type(file: &Path) -> &'static str {
    let ending = file
        .extension()
        .map(|e| e.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    match ending.as_str() {
        "html" | "htm" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "wasm" => "application/wasm",
        "css" => "text/css; charset=utf-8",
        "json" | "webmanifest" => "application/json",
        "txt" => "text/plain; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    }
}

/// The bytes of a request target's path with each `%xx` decoded, or none when one is malformed.
fn percent_decode(path: &str) -> Option<Vec<u8>> {
    let bytes = path.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let digit = |at: usize| char::from(*bytes.get(at)?).to_digit(16);
            out.push(u8::try_from(digit(i + 1)? * 16 + digit(i + 2)?).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    Some(out)
}

/// The file of `folder` that the request target `target` names, or none when the target names nothing inside the
/// folder. The query is dropped and the path decoded first. A path that ends in `/` names its `index.html`. A part
/// that is `.` or `..`, an empty part, and a part with `\`, `:` or a control character are refused, so no file
/// outside the folder is ever looked at.
pub fn resolve(folder: &Path, target: &str) -> Option<PathBuf> {
    let path = target.split(['?', '#']).next().unwrap_or_default();
    #[expect(
        clippy::disallowed_methods,
        reason = "a URL's path is UTF-8 text, not a program's bytes"
    )]
    let path = String::from_utf8(percent_decode(path)?).ok()?;
    let mut parts: Vec<&str> = path.strip_prefix('/')?.split('/').collect();
    if parts.last() == Some(&"") {
        parts.pop();
        parts.push("index.html");
    }
    let mut file = folder.to_path_buf();
    for part in parts {
        let bad = |c: char| c == '\\' || c == ':' || c.is_control();
        if part.is_empty() || part == "." || part == ".." || part.contains(bad) {
            return None;
        }
        file.push(part);
    }
    file.is_file().then_some(file)
}

/// What a request with `method` and `target` gets from `folder`: the file for `GET` and `HEAD`, 404 when the
/// target names no file of the folder, 405 for another method.
pub fn respond(folder: &Path, method: &str, target: &str) -> Answer {
    if method != "GET" && method != "HEAD" {
        return Answer::Status(405, "Method Not Allowed");
    }
    match resolve(folder, target) {
        Some(file) => Answer::File(file),
        None => Answer::Status(404, "Not Found"),
    }
}

/// The head of a response: the status line and the headers, ending in an empty line. Every response says that it
/// is to be checked again before a cached copy is used, so that a new build is seen at the next reload, and closes
/// the connection. `isolation` adds [`ISOLATION_HEADERS`].
pub fn head(status: u16, reason: &str, content_type: &str, length: u64, isolation: bool) -> String {
    let mut head = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\
         Cache-Control: no-cache\r\nConnection: close\r\n"
    );
    if status == 405 {
        head.push_str("Allow: GET, HEAD\r\n");
    }
    if isolation {
        for (name, value) in ISOLATION_HEADERS {
            head.push_str(&format!("{name}: {value}\r\n"));
        }
    }
    head.push_str("\r\n");
    head
}

/// The method and the target of a request's first line, as `GET /game.wasm HTTP/1.1`.
fn request_line(head: &str) -> Option<(&str, &str)> {
    let mut words = head.lines().next()?.split(' ');
    let (method, target, version) = (words.next()?, words.next()?, words.next()?);
    (version.starts_with("HTTP/") && words.next().is_none()).then_some((method, target))
}

/// Reads one request from `stream` and writes its response: a file of `folder`, or a status. `isolation` as in
/// [`head`]. A request head that is too long, or not a request, gets status 400.
pub fn handle(folder: &Path, stream: &mut (impl Read + Write), isolation: bool) -> std::io::Result<()> {
    let mut request = Vec::new();
    let mut buffer = [0u8; 1024];
    while !request.windows(4).any(|w| w == b"\r\n\r\n") && request.len() <= HEAD_LIMIT {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        request.extend_from_slice(&buffer[..n]);
    }
    #[expect(
        clippy::disallowed_methods,
        reason = "an HTTP request's head is text; one that is not UTF-8 is refused"
    )]
    let text = String::from_utf8(request).unwrap_or_default();
    let (method, answer) = match request_line(&text) {
        Some((method, target)) if text.len() <= HEAD_LIMIT => (method, respond(folder, method, target)),
        _ => ("GET", Answer::Status(400, "Bad Request")),
    };
    match answer {
        Answer::File(path) => {
            let mut file = std::fs::File::open(&path)?;
            let length = file.metadata()?.len();
            stream.write_all(head(200, "OK", content_type(&path), length, isolation).as_bytes())?;
            if method != "HEAD" {
                std::io::copy(&mut file, stream)?;
            }
        }
        Answer::Status(status, reason) => {
            let body = format!("{status} {reason}\n");
            let text = "text/plain; charset=utf-8";
            stream.write_all(head(status, reason, text, body.len() as u64, isolation).as_bytes())?;
            if method != "HEAD" {
                stream.write_all(body.as_bytes())?;
            }
        }
    }
    stream.flush()
}

/// A listener on `127.0.0.1` and its port: `port` when one is named, else the first free port from
/// [`DEFAULT_PORT`] to [`LAST_PORT`]. A named port that is taken is an error.
pub fn bind(port: Option<u16>) -> Result<(TcpListener, u16), String> {
    let listen = |port: u16| TcpListener::bind((Ipv4Addr::LOCALHOST, port));
    match port {
        Some(port) => listen(port)
            .map(|l| (l, port))
            .map_err(|e| format!("cannot serve on port {port} of 127.0.0.1: {e}")),
        None => (DEFAULT_PORT..=LAST_PORT)
            .find_map(|port| listen(port).ok().map(|l| (l, port)))
            .ok_or_else(|| {
                format!(
                    "cannot serve: the ports {DEFAULT_PORT} to {LAST_PORT} of 127.0.0.1 are all taken; name another \
                     with --port <n>"
                )
            }),
    }
}

/// The address a browser opens for a server on `port`.
pub fn address(port: u16) -> String {
    format!("http://127.0.0.1:{port}/")
}

/// Answers the requests that reach `listener` from `folder`, each on a thread of its own, until the process is
/// interrupted. A connection that sends nothing for ten seconds is dropped.
pub fn serve(listener: &TcpListener, folder: &Path, isolation: bool) {
    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let folder = folder.to_path_buf();
        std::thread::spawn(move || {
            let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
            // A browser that closes a connection early is no error of the server's.
            let _ = handle(&folder, &mut stream, isolation);
            let _ = stream.shutdown(std::net::Shutdown::Both);
        });
    }
}

/// Opens `url` in the user's browser, with the system's opener.
pub fn open_browser(url: &str) -> Result<(), String> {
    let mut command = if cfg!(windows) {
        let mut c = std::process::Command::new("cmd");
        // The empty string is the window title `start` takes its first quoted argument for.
        c.args(["/c", "start", "", url]);
        c
    } else {
        let mut c = std::process::Command::new("xdg-open");
        c.arg(url);
        c
    };
    command
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| format!("cannot open the browser: {e}; open {url} yourself"))
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(format!("cannot open the browser; open {url} yourself"))
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A stream that gives a request and keeps the response.
    struct Pipe {
        request: std::io::Cursor<Vec<u8>>,
        response: Vec<u8>,
    }

    impl Read for Pipe {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.request.read(buf)
        }
    }

    impl Write for Pipe {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.response.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    /// The response to `request` from `folder`, as its head and its body.
    fn ask(folder: &Path, request: &str, isolation: bool) -> (String, Vec<u8>) {
        let mut pipe = Pipe {
            request: std::io::Cursor::new(request.as_bytes().to_vec()),
            response: Vec::new(),
        };
        handle(folder, &mut pipe, isolation).unwrap();
        let at = pipe.response.windows(4).position(|w| w == b"\r\n\r\n").expect("a head");
        // A response's head is ASCII: each byte is its character.
        (
            pipe.response[..at].iter().map(|&b| char::from(b)).collect(),
            pipe.response[at + 4..].to_vec(),
        )
    }

    /// A web folder in a project folder: the folder, and a file outside it.
    fn web() -> (tempfile::TempDir, PathBuf) {
        let d = tempfile::tempdir().unwrap();
        let web = d.path().join("build").join("web");
        std::fs::create_dir_all(web.join("sub")).unwrap();
        std::fs::write(d.path().join("qb64rust.toml"), "secret").unwrap();
        std::fs::write(d.path().join("build").join("game.exe"), "exe").unwrap();
        std::fs::write(web.join("index.html"), "<!doctype html>").unwrap();
        std::fs::write(web.join("game.wasm"), [0u8, 0x61, 0x73, 0x6d]).unwrap();
        std::fs::write(web.join("game.js"), "var a;").unwrap();
        std::fs::write(web.join("a b.data"), "data").unwrap();
        std::fs::write(web.join("sub").join("index.html"), "sub").unwrap();
        (d, web)
    }

    /// Spec `compiler/web-serve`, "Serving a web folder": `/` is `index.html`, a path is its file, and a path
    /// outside the folder, a missing file and a folder without a slash are 404; another method is 405.
    #[test]
    fn serves_only_the_folder() {
        let (_d, web) = web();
        assert_eq!(respond(&web, "GET", "/"), Answer::File(web.join("index.html")));
        assert_eq!(respond(&web, "HEAD", "/game.wasm"), Answer::File(web.join("game.wasm")));
        assert_eq!(
            respond(&web, "GET", "/game.js?v=2#top"),
            Answer::File(web.join("game.js"))
        );
        assert_eq!(respond(&web, "GET", "/a%20b.data"), Answer::File(web.join("a b.data")));
        assert_eq!(
            respond(&web, "GET", "/sub/"),
            Answer::File(web.join("sub").join("index.html"))
        );
        let missing = Answer::Status(404, "Not Found");
        for target in [
            "/../qb64rust.toml",
            "/../../qb64rust.toml",
            "/%2e%2e/qb64rust.toml",
            "/..%2fqb64rust.toml",
            "/..%5c..%5cqb64rust.toml",
            "/sub/../../game.exe",
            "/./index.html",
            "//index.html",
            "/X:/secret.txt",
            "/x%3a/secret.txt",
            "/missing.html",
            "/sub",
            "/%zz",
            "/%",
            "index.html",
            "",
        ] {
            assert_eq!(respond(&web, "GET", target), missing, "{target}");
        }
        for method in ["POST", "PUT", "DELETE", "get"] {
            assert_eq!(
                respond(&web, method, "/"),
                Answer::Status(405, "Method Not Allowed"),
                "{method}"
            );
        }
    }

    /// Spec `compiler/web-serve`, "Headers and content types": the two headers and `no-cache` on every answer, the
    /// content types, and `--no-headers` without the two.
    #[test]
    fn headers_and_content_types() {
        let (_d, web) = web();
        let (head, body) = ask(&web, "GET /game.wasm HTTP/1.1\r\nHost: x\r\n\r\n", true);
        assert!(head.starts_with("HTTP/1.1 200 OK\r\n"), "{head}");
        for line in [
            "Content-Type: application/wasm",
            "Content-Length: 4",
            "Cache-Control: no-cache",
            "Cross-Origin-Opener-Policy: same-origin",
            "Cross-Origin-Embedder-Policy: require-corp",
        ] {
            assert!(head.lines().any(|l| l == line), "{line}: {head}");
        }
        assert_eq!(body, [0u8, 0x61, 0x73, 0x6d]);
        // `HEAD` has the head of `GET` and no body.
        let (head2, body) = ask(&web, "HEAD /game.wasm HTTP/1.1\r\n\r\n", true);
        assert_eq!((head2, body.len()), (head, 0));
        // An error answer has the headers too, so that a page's own failed request is not a second problem.
        let (head, body) = ask(&web, "GET /../qb64rust.toml HTTP/1.1\r\n\r\n", true);
        assert!(head.starts_with("HTTP/1.1 404 Not Found\r\n"), "{head}");
        assert!(head.contains("Cross-Origin-Embedder-Policy: require-corp"), "{head}");
        assert_eq!(body, b"404 Not Found\n");
        let (head, _) = ask(&web, "POST / HTTP/1.1\r\n\r\n", true);
        assert!(
            head.starts_with("HTTP/1.1 405 ") && head.contains("Allow: GET, HEAD"),
            "{head}"
        );
        let (head, _) = ask(&web, "nonsense\r\n\r\n", true);
        assert!(head.starts_with("HTTP/1.1 400 "), "{head}");
        let long = format!("GET /{} HTTP/1.1\r\n\r\n", "a".repeat(HEAD_LIMIT));
        assert!(ask(&web, &long, true).0.starts_with("HTTP/1.1 400 "));
        // As a host that cannot send the two headers.
        let (head, _) = ask(&web, "GET / HTTP/1.1\r\n\r\n", false);
        assert!(
            head.contains("Content-Type: text/html") && !head.contains("Cross-Origin"),
            "{head}"
        );
        for (file, expected) in [
            ("index.html", "text/html; charset=utf-8"),
            ("game.js", "text/javascript; charset=utf-8"),
            ("GAME.WASM", "application/wasm"),
            ("game.data", "application/octet-stream"),
            ("_headers", "application/octet-stream"),
        ] {
            assert_eq!(content_type(Path::new(file)), expected, "{file}");
        }
    }

    #[test]
    fn the_address_of_a_port() {
        assert_eq!(address(8064), "http://127.0.0.1:8064/");
    }
}
