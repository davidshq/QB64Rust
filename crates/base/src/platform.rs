//! The platforms a build targets (spec `compiler/platforms`, design D3 of `m7-linux`): Windows x64 and Linux x64.
//! A build targets the platform the compiler runs on; [`Platform::host`] is the only place that reads it, so that
//! everything else takes the platform as a value and the rules of both can be tested on either.

/// A platform a program is built for. The default is Windows, the platform of every value made before Linux was
/// one (a `Resolved` of the codegen's tests).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Platform {
    #[default]
    Windows,
    Linux,
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

    /// The extension of an executable, without the dot: `exe` on Windows, none on Linux, as `qb64pe` names them.
    pub fn exe_extension(self) -> &'static str {
        match self {
            Platform::Windows => "exe",
            Platform::Linux => "",
        }
    }

    /// The name in messages.
    pub fn name(self) -> &'static str {
        match self {
            Platform::Windows => "Windows",
            Platform::Linux => "Linux",
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
    }

    #[test]
    fn the_host() {
        assert_eq!(Platform::host() == Platform::Windows, cfg!(windows));
    }
}
