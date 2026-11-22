//! The platforms a build targets (spec `compiler/platforms`, design D3 of `m7-linux`): Windows x64 and Linux x64,
//! and WebAssembly when `--target wasm` names it (spec `compiler/wasm-target`, design D1 of `wasm-console`).
//! [`Platform::host`] is the only place that reads the platform the compiler runs on, so that everything else
//! takes the platform as a value and the rules of each can be tested on any.

/// A platform a program is built for. The default is Windows, the platform of every value made before Linux was
/// one (a `Resolved` of the codegen's tests).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Platform {
    #[default]
    Windows,
    Linux,
    /// `wasm32` under Emscripten, run by Node. Never the host; chosen only by `--target wasm`.
    Wasm,
}

impl Platform {
    /// The platform the compiler runs on. Off Windows this is Linux, the rules of which a build on another system
    /// would follow; a build there is refused first ([`Platform::host_is_supported`]).
    pub fn host() -> Platform {
        if cfg!(windows) {
            Platform::Windows
        } else {
            Platform::Linux
        }
    }

    /// Whether the compiler runs on a platform it can build for (spec `compiler/platforms`, "Supported platforms").
    pub fn host_is_supported() -> bool {
        cfg!(all(
            target_arch = "x86_64",
            any(target_os = "windows", target_os = "linux")
        ))
    }

    /// The extension of an executable, without the dot: `exe` on Windows, none on Linux, as `qb64pe` names them;
    /// `js` for the wasm target, whose `.wasm` file has the same name beside it.
    pub fn exe_extension(self) -> &'static str {
        match self {
            Platform::Windows => "exe",
            Platform::Linux => "",
            Platform::Wasm => "js",
        }
    }

    /// The platform whose C++ toolchain a build for this one runs: its own, or the host's for the wasm target,
    /// which takes only `make` from it (design D2 of `wasm-console`). Never [`Platform::Wasm`].
    pub fn toolchain(self) -> Platform {
        match self {
            Platform::Wasm => Platform::host(),
            other => other,
        }
    }

    /// The size of a pointer in a program built for this platform, in bytes: what an `_OFFSET` takes, and libqb's
    /// `ptrszint` (spec `compiler/wasm-target`, "A pointer is 4 bytes on this target"; design D1 of
    /// `wasm-pointer-width`).
    pub fn pointer_bytes(self) -> u32 {
        match self {
            Platform::Windows | Platform::Linux => 8,
            Platform::Wasm => 4,
        }
    }

    /// The name in messages.
    pub fn name(self) -> &'static str {
        match self {
            Platform::Windows => "Windows",
            Platform::Linux => "Linux",
            Platform::Wasm => "wasm",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// The default executable name of spec `compiler/cli`: the source's with the platform's extension.
    #[test]
    fn executable_names() {
        let src = Path::new("dir/game.bas");
        assert_eq!(
            src.with_extension(Platform::Windows.exe_extension()),
            Path::new("dir/game.exe")
        );
        assert_eq!(
            src.with_extension(Platform::Linux.exe_extension()),
            Path::new("dir/game")
        );
        assert_eq!(
            src.with_extension(Platform::Wasm.exe_extension()),
            Path::new("dir/game.js")
        );
    }

    #[test]
    fn names() {
        assert_eq!(Platform::Windows.name(), "Windows");
        assert_eq!(Platform::Linux.name(), "Linux");
        assert_eq!(Platform::Wasm.name(), "wasm");
    }

    #[test]
    fn pointer_sizes() {
        assert_eq!(Platform::Windows.pointer_bytes(), 8);
        assert_eq!(Platform::Linux.pointer_bytes(), 8);
        assert_eq!(Platform::Wasm.pointer_bytes(), 4);
    }

    #[test]
    fn the_toolchain() {
        assert_eq!(Platform::Windows.toolchain(), Platform::Windows);
        assert_eq!(Platform::Linux.toolchain(), Platform::Linux);
        assert_eq!(Platform::Wasm.toolchain(), Platform::host());
    }

    #[test]
    fn the_host() {
        assert_eq!(Platform::host() == Platform::Windows, cfg!(windows));
        assert_ne!(Platform::host(), Platform::Wasm);
    }
}
