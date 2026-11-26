//! The compile of one main file, from its bytes to a dump, `-z`'s C++ or an executable (design D3 of
//! `basic-unit-tests`): the part of `qb64rust`'s `run()` after the options, the main file and its fixes are known,
//! moved here so that `qb64rust test` builds through the same path. What it prints goes to the caller's sink, which
//! `run()` prints as it comes.

use crate::lints::Lints;
use crate::{
    Frontend, Instrument, Resolved, build, cpp_errors, dump_cpp, dump_debug_info, dump_ir, emit_ir_debug, emit_ir_with,
    exe_icon_path, frontend_explicit, library, log_folder, lower, toolchain, used_embedded_files,
};
use qb64rust_base::Platform;
use qb64rust_base::fixes::Fixes;
use qb64rust_base::layout::Settings;
use qb64rust_syntax::Loader;
use std::path::{Path, PathBuf};

/// The options of a compile, as the command line gave them.
#[derive(Default)]
pub struct CompileOptions {
    /// The executable to write.
    pub exe: PathBuf,
    pub quiet: bool,
    /// `-z`.
    pub cpp_only: bool,
    /// `--print-build-inputs` (with `cpp_only`): everything a build writes before `make`, then the program's half
    /// of the build's key printed in place of the build (design D1 of `testing-reset`).
    pub print_build_inputs: bool,
    /// `--dump <stage>`, other than `tokens`.
    pub dump: Option<String>,
    /// `--qb64pe-root`.
    pub root: Option<PathBuf>,
    /// `--toolchain-root`.
    pub toolchain_root: Option<PathBuf>,
    /// `--libqb-root`.
    pub libqb_root: Option<PathBuf>,
    /// The compiler root: `--include-root`, or the folder of the executable.
    pub include_root: PathBuf,
    pub keep_build: bool,
    pub coverage: bool,
    /// `--trace-values`: a record call at each store into a variable (OpenSpec change `value-trace`).
    pub trace: bool,
    /// A call stack in a runtime error's message (OpenSpec change `call-trace`): `--call-trace`, `[build]`
    /// `call_trace`, or `--debug`, as `settings::Build::call_trace` decides.
    pub call_trace: bool,
    pub debug: bool,
    /// `-w`.
    pub warnings: bool,
    /// `-e`: `OPTION _EXPLICIT` forced, as `qb64pe -e` does.
    pub explicit: bool,
    /// `-f:<setting>=<value>` as given, checked by [`build_settings`].
    pub settings: Vec<String>,
    /// The program's fixes.
    pub fixes: Fixes,
    /// The lints that are on (spec `compiler/lints`); the warnings of the others are dropped.
    pub lints: Lints,
    /// `--target wasm`; `None` for the host (spec `compiler/cli`, "The target option").
    pub target: Option<Platform>,
    /// `--emsdk-root`: the Emscripten SDK of a wasm build.
    pub emsdk_root: Option<PathBuf>,
    /// The files a window program's page ships, with their paths in the output folder: those of `[build]` `copy`
    /// in a build without `-o` (`output::copied_files`), else none (design D11 of `wasm-window`).
    pub page_files: Vec<(String, PathBuf)>,
    /// A build without `-o`: a window program of the wasm target is written into a web folder (OpenSpec change
    /// `wasm-publish`, design D1).
    pub web: bool,
}

impl CompileOptions {
    /// The platform the program is compiled for: the target named, else the host.
    pub fn platform(&self) -> Platform {
        self.target.unwrap_or_else(Platform::host)
    }
}

/// The `-f:` settings: `OptimizeCppProgram=true` (as `qb64pe` turns it into `-O2`) and `ExtraLinkerFlags=<value>`
/// (added to the link command), `StripDebugSymbols=false` (the executable unstripped) and `GenerateLicenseFile`;
/// the layout settings of `-y`, checked and ignored as `qb64pe` ignores them in a build; any other setting is not
/// supported yet (spec `compiler/cli`).
pub fn build_settings(settings: &[String]) -> Result<build::Settings, String> {
    let mut out = build::Settings::default();
    let mut layout = Settings::default();
    for s in settings {
        let (name, value) = s.split_once('=').unwrap_or((s, ""));
        if layout.set(name, value)? {
            continue;
        }
        match name {
            "OptimizeCppProgram" => {
                out.optimize = match value.to_ascii_lowercase().as_str() {
                    "true" => true,
                    "false" => false,
                    _ => return Err(format!("setting `{name}` needs `true` or `false`, not `{value}`")),
                }
            }
            "GenerateLicenseFile" => {
                out.license = match value.to_ascii_lowercase().as_str() {
                    "true" => true,
                    "false" => false,
                    _ => return Err(format!("setting `{name}` needs `true` or `false`, not `{value}`")),
                }
            }
            "ExtraLinkerFlags" => out.extra_link = value.to_string(),
            "StripDebugSymbols" => {
                out.keep_symbols = match value.to_ascii_lowercase().as_str() {
                    "true" => false,
                    "false" => true,
                    _ => return Err(format!("setting `{name}` needs `true` or `false`, not `{value}`")),
                }
            }
            _ => return Err(format!("setting `{name}` is not supported yet")),
        }
    }
    Ok(out)
}

/// The executable an earlier build left, and its debug symbol file with it, so that a file beside the executable is
/// never an older build's (spec `compiler/debug-info`). For the wasm target the `.wasm` file beside the `.js` goes
/// too (spec `compiler/cli`, "The output of a wasm build").
pub fn remove_old(exe: &Path, platform: Platform) {
    let _ = std::fs::remove_file(exe);
    let _ = std::fs::remove_file(build::debug_path(exe));
    if platform == Platform::Wasm {
        let _ = std::fs::remove_file(exe.with_extension("wasm"));
        // A page's files go with it (OpenSpec change `wasm-window`); the page itself is written or removed by the
        // build.
        let _ = std::fs::remove_file(exe.with_extension("data"));
    }
}

/// Compiles the main file `input` (shown as `name`, its text `bytes`, its includes read by `loader`) as `o` says:
/// a dump, `-z`'s C++ folder, or the executable `o.exe`. What the command line prints goes to `out`. `Ok(true)` when
/// it worked, `Ok(false)` when the program has errors (printed to `out`); `Err` for a failure of the compiler or the
/// build.
pub fn compile_exe(
    input: &Path,
    name: &str,
    bytes: Vec<u8>,
    loader: impl FnOnce(qb64rust_base::FileId) -> Box<dyn Loader>,
    o: &CompileOptions,
    out: &mut dyn FnMut(&str),
) -> Result<bool, String> {
    let exe = &o.exe;
    let include_root = &o.include_root;
    let platform = o.platform();
    let remove_old = || remove_old(exe, platform);
    let mut fe = frontend_explicit(
        platform,
        o.explicit,
        name,
        bytes,
        loader,
        Some((input, include_root)),
        o.fixes,
    );
    o.lints.drop_from(&mut fe.diagnostics);
    // The files of the program's `DECLARE LIBRARY` blocks, looked for from the QB64pe folder, else the include root,
    // as the old compiler's folder (design D5 of `m6-toolchain`), for a build, `-z` and the `cpp` and `debug-info`
    // dumps; the other dumps need none, nor does a block with no name. Only a block without a header needs the
    // toolchain (its `nm`).
    let resolved = if matches!(o.dump.as_deref(), None | Some("cpp" | "debug-info"))
        && !fe.has_errors()
        && fe.program.libraries.iter().any(|l| !l.names.is_empty())
    {
        let places = build::find_root(o.root.as_deref()).and_then(|root| {
            let nm = toolchain::find(platform, o.toolchain_root.as_deref(), o.root.as_deref())
                .map(|t| t.tool(toolchain::Tool::Nm));
            library::Places::new(root.as_deref().unwrap_or(include_root), input, nm)
                .map(|places| library::Places { platform, ..places })
                .map_err(|e| e.to_string())
        });
        let places = places.inspect_err(|_| {
            if o.dump.is_none() && !o.cpp_only {
                remove_old();
            }
        })?;
        library::resolve(&mut fe, &places)
    } else {
        // No library to look for; the platform still shapes the C++ (a debug build's end of a procedure, `procs.rs`).
        Resolved {
            platform,
            ..Resolved::default()
        }
    };
    let errors = fe.diagnostics.error_count();
    let instrument = Instrument {
        coverage: o.coverage,
        trace: o.trace,
        call_trace: o.call_trace,
    };
    // Warnings only with -w, never counted in the summary (spec `compiler/cli`).
    let report = |fe: &Frontend, out: &mut dyn FnMut(&str)| {
        let errors = fe.diagnostics.error_count();
        if o.warnings {
            out(&fe.render_with_warnings());
        } else if errors > 0 {
            out(&fe.render_diagnostics());
        }
        if errors > 0 {
            out(&format!("{}\n", fe.diagnostics.summary()));
        }
    };
    if let Some(stage) = o.dump.as_deref() {
        match stage {
            "tree" => out(&qb64rust_syntax::dump_trees(&fe.parsed, &fe.map)),
            _ if errors > 0 => {}
            "typed" => out(&qb64rust_sema::dump_typed(&fe.program)),
            "ir" => out(&dump_ir(&fe)),
            "cpp" => out(&dump_cpp(&fe, &resolved, instrument)),
            "debug-info" => out(&dump_debug_info(&fe, &resolved)),
            _ => unreachable!(),
        }
        report(&fe, out);
        return Ok(errors == 0);
    }

    let mut settings = build_settings(&o.settings).inspect_err(|_| remove_old())?;
    settings.debug = o.debug;
    settings.fixes = o.fixes;
    for lib in &resolved.libraries {
        settings.libraries.extend(lib.files.iter().cloned());
        settings.link_options.extend(lib.link_options.iter().cloned());
        settings.headers.extend(lib.headers.iter().cloned());
    }
    if fe.diagnostics.error_count() > 0 {
        remove_old();
        report(&fe, out);
        return Ok(false);
    }
    report(&fe, out);

    let ir = lower(&fe);
    settings.flags = build::make_flags(&ir.settings);
    // The files a window program's page ships: those `[build]` `copy` copies (OpenSpec change `wasm-window`, design
    // D11).
    if platform == Platform::Wasm && ir.settings.window {
        settings.wasm_files = o.page_files.clone();
    }
    // A window program built without `-o` goes into the web folder, `web` in the output folder, which is uploaded
    // whole (OpenSpec change `wasm-publish`, design D1). Only the lowered program says whether it has a window.
    settings.web = platform == Platform::Wasm && ir.settings.window && o.web;
    let script = match exe.file_name() {
        Some(name) if settings.web => build::web_folder(exe).join(name),
        _ => exe.clone(),
    };
    let page = script.with_file_name(build::WASM_INDEX);
    settings.uncached = ir.settings.compile_stamp;
    // A log statement names a relative source file from the working folder, as the old compiler does from its
    // start folder (design D2 of `m4-handlers-logging-checking`).
    let log_folder = std::env::current_dir().ok().map(|d| log_folder(&d));
    // A debug build's symbol file comes from the same emitter as its fragments (design D3 of `m5-debug-build-symbols`).
    let (fragments, debug_info) = if o.debug {
        let (fragments, info) = emit_ir_debug(&fe, &ir, &resolved, instrument, log_folder.as_deref());
        (fragments, Some(info))
    } else {
        (
            emit_ir_with(&fe, &ir, &resolved, instrument, log_folder.as_deref()),
            None,
        )
    };
    let build_dir = build::build_dir(exe);
    if o.cpp_only && !o.print_build_inputs {
        let temp = build::write_fragments(&build_dir, &fragments).map_err(|e| e.to_string())?;
        out(&format!("{}\n", temp.display()));
        return Ok(true);
    }
    // Before any step that can fail, so a failed build never leaves an older executable or symbol file behind.
    if !o.print_build_inputs {
        remove_old();
        if settings.web {
            self::remove_old(&script, platform);
            let _ = std::fs::remove_file(&page);
            // The page an earlier version wrote beside the script (OpenSpec change `wasm-window`).
            let _ = std::fs::remove_file(exe.with_extension("html"));
            let folder = build::web_folder(exe);
            std::fs::create_dir_all(&folder).map_err(|e| format!("creating {}: {e}", folder.display()))?;
        }
    }
    if !Platform::host_is_supported() {
        return Err(
            "building an executable needs Windows x64 or Linux x64; checking (-z, --dump, -y, lsp) works here".into(),
        );
    }
    // The SDK of a wasm build first: without it nothing else of the build is looked for (spec `compiler/toolchain`,
    // "The Emscripten SDK").
    let emsdk = match platform {
        Platform::Wasm => Some(toolchain::find_emsdk(o.emsdk_root.as_deref())?),
        Platform::Windows | Platform::Linux => None,
    };
    // A named folder that is not QB64pe's is reported before the toolchain is looked for.
    let qb64pe = build::find_root(o.root.as_deref())?;
    let toolchain = toolchain::find(platform, o.toolchain_root.as_deref(), o.root.as_deref())?;
    let mut libqb = build::find_libqb(o.libqb_root.as_deref())?;
    // A wasm build builds in a tree of its own, made current from the copy first (design D3 of `wasm-console`).
    if emsdk.is_some() && !o.print_build_inputs {
        let tree = build::wasm_libqb(&libqb)?;
        build::sync_wasm_libqb(&libqb, &tree)?;
        libqb = tree;
    }
    let roots = build::Roots {
        qb64pe,
        toolchain,
        include: include_root.clone(),
        libqb,
        emsdk,
    };
    build::write_fragments(&build_dir, &fragments).map_err(|e| e.to_string())?;
    let icon = exe_icon_path(&fe, input, include_root);
    let exe_name = exe
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    // The icon and version resources are Windows' (`windres`); on Linux the old compiler writes none.
    if platform == Platform::Windows {
        build::write_resources(
            &build_dir,
            icon.as_deref(),
            ir.settings.version_info.as_ref(),
            &exe_name,
        )?;
    }
    build::write_embedded(&build_dir, &used_embedded_files(&fe, input, include_root)?)?;
    if o.print_build_inputs {
        // A program whose executable holds its build time is built each time: it has no key to print.
        let key = if settings.uncached {
            "uncached".to_string()
        } else {
            build::program_inputs_key(&roots, &build_dir, &settings)
        };
        out(&format!("{key}\n"));
        return Ok(true);
    }
    if let Err(e) = build::build(&roots, &build_dir, &script, o.quiet, &settings) {
        // A failure that names only library functions is the program's (design D4 of `cpp-failure-errors`): its
        // errors as the front end's, with the summary; the build folder goes as after a build that worked.
        if let build::BuildError::Toolchain { output, .. } = &e
            && let Some(errors) = cpp_errors::diagnostics(&fe, output)
        {
            for d in errors {
                fe.diagnostics.push(d);
            }
            out(&fe.render_diagnostics());
            out(&format!("{}\n", fe.diagnostics.summary()));
            if !o.keep_build {
                let _ = std::fs::remove_dir_all(&build_dir);
            }
            return Ok(false);
        }
        // A header or library of the system packages missing on Linux: the packages named after the toolchain's
        // output (spec `compiler/toolchain`, "The system packages").
        if let build::BuildError::Toolchain { output, .. } = &e
            && Platform::host() == Platform::Linux
            && let Some(file) = cpp_errors::missing_system_file(output)
        {
            return Err(format!(
                "{e}\n{file} is missing. {}",
                toolchain::packages_message(toolchain::os_release().as_deref())
            ));
        }
        return Err(e.to_string());
    }
    // Beside the executable once it is in place; any other build has removed an earlier one with the executable.
    if let Some(info) = debug_info {
        let path = build::debug_path(exe);
        std::fs::write(&path, info.to_json()).map_err(|e| format!("writing {}: {e}", path.display()))?;
    }
    if !o.keep_build {
        let _ = std::fs::remove_dir_all(&build_dir);
    }
    if !o.quiet {
        // A web folder is named by its page (spec `compiler/cli`, "The output of a wasm build").
        let written = if settings.web { &page } else { exe };
        out(&format!("{}\n", written.display()));
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec `compiler/cli`, "The output of a wasm build", scenario "A failed build": the `.js` and the `.wasm` of an
    /// earlier build are both gone. On the host a `.wasm` beside the executable is not the build's, and stays.
    #[test]
    fn a_failed_wasm_build_removes_both_files() {
        let d = tempfile::tempdir().unwrap();
        let (js, wasm) = (d.path().join("p.js"), d.path().join("p.wasm"));
        std::fs::write(&js, "old").unwrap();
        std::fs::write(&wasm, "old").unwrap();
        remove_old(&js, Platform::Wasm);
        assert!(!js.exists() && !wasm.exists());
        let exe = d.path().join("p.exe");
        std::fs::write(&exe, "old").unwrap();
        std::fs::write(&wasm, "other").unwrap();
        remove_old(&exe, Platform::Windows);
        assert!(!exe.exists() && wasm.exists());
    }
}
