//! The output folder of a build without `-o` (spec `compiler/cli`, "The output folder" and "Files copied into the
//! output folder"; design D3 of `build-output-folder`). Before the compile, the folder is made and each `[build]`
//! `copy` entry is copied into it: a file when the output folder has none of that name or the source's is newer,
//! a folder file by file by the same rule. Nothing in the output folder is deleted.
//!
//! The copy is given the source's modification time here, so that "newer" means the same on every system.
//! `std::fs::copy` does not promise it: on Linux it copies the contents only (`library/std/src/sys/fs/unix.rs`,
//! `copy`: `open_to_and_set_permissions` and `io::copy`), so the copy would get the time of the copy; on Windows it
//! is `CopyFileExW` (`sys/fs/windows.rs`). With the time set, a file the program rewrote in the output folder is
//! newer than its source and is kept, and a source edited after the build is newer than its copy and goes over it.
//! The bytes are written into the file rather than copied with `std::fs::copy`, which would copy a read-only
//! attribute as well: the program could then not rewrite its file, nor the next build replace it.

use crate::settings::Build;
use std::path::Path;

/// Makes the output folder of `build` and copies its `copy` entries into it. An error names the settings file and
/// the entry's line for an entry that does not exist, and the file for one that cannot be read or written.
pub fn prepare(build: &Build) -> Result<(), String> {
    for entry in &build.copy {
        if !entry.path.exists() {
            let file = build.root.join(crate::settings::SETTINGS_FILE);
            return Err(format!(
                "{}:{}: `copy` names \"{}\", and there is no such file or folder",
                file.display(),
                entry.line,
                entry.name
            ));
        }
    }
    std::fs::create_dir_all(&build.output).map_err(|e| format!("creating {}: {e}", build.output.display()))?;
    for entry in &build.copy {
        copy_newer(&entry.path, &build.output.join(&entry.name))?;
    }
    Ok(())
}

/// Copies the file or folder `from` to `to` where `to` is missing or older ([`prepare`]).
fn copy_newer(from: &Path, to: &Path) -> Result<(), String> {
    let meta = std::fs::metadata(from).map_err(|e| format!("reading {}: {e}", from.display()))?;
    if meta.is_dir() {
        std::fs::create_dir_all(to).map_err(|e| format!("creating {}: {e}", to.display()))?;
        let entries = std::fs::read_dir(from).map_err(|e| format!("reading {}: {e}", from.display()))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("reading {}: {e}", from.display()))?;
            copy_newer(&entry.path(), &to.join(entry.file_name()))?;
        }
        return Ok(());
    }
    let source_time = meta
        .modified()
        .map_err(|e| format!("reading {}: {e}", from.display()))?;
    if let Ok(copy_time) = std::fs::metadata(to).and_then(|m| m.modified())
        && copy_time >= source_time
    {
        return Ok(());
    }
    let write = |e: std::io::Error| format!("copying {} to {}: {e}", from.display(), to.display());
    if let Some(dir) = to.parent() {
        std::fs::create_dir_all(dir).map_err(write)?;
    }
    let mut source = std::fs::File::open(from).map_err(|e| format!("reading {}: {e}", from.display()))?;
    let mut copy = std::fs::File::create(to).map_err(write)?;
    std::io::copy(&mut source, &mut copy).map_err(write)?;
    copy.set_modified(source_time).map_err(write)?;
    Ok(())
}

/// The files of `build`'s `copy` entries, each with its path in the output folder (`/` between folders): a file
/// entry as itself, a folder entry with every file below it. Sorted by that path. A window program of the wasm
/// target packs them into its page's data file (design D11 of `wasm-window`), since a page cannot read the files
/// beside it. An entry that does not exist gives nothing; [`prepare`] has reported it before the compile.
pub fn copied_files(build: &Build) -> Vec<(String, std::path::PathBuf)> {
    fn walk(path: &Path, name: String, out: &mut Vec<(String, std::path::PathBuf)>) {
        if path.is_dir() {
            for entry in std::fs::read_dir(path).into_iter().flatten().flatten() {
                let child = format!("{name}/{}", entry.file_name().to_string_lossy());
                walk(&entry.path(), child, out);
            }
        } else if path.is_file() {
            out.push((name, path.to_path_buf()));
        }
    }
    let mut out = Vec::new();
    for entry in &build.copy {
        let name = entry.name.replace('\\', "/").trim_end_matches('/').to_string();
        walk(&entry.path, name, &mut out);
    }
    out.sort();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::CopyEntry;
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime};

    /// A project in `dir` whose `copy` lists `names`, built into `dir/build`.
    fn project(dir: &Path, names: &[&str]) -> Build {
        Build {
            root: dir.to_path_buf(),
            output: dir.join("build"),
            copy: names
                .iter()
                .map(|n| CopyEntry {
                    name: (*n).to_string(),
                    path: dir.join(n),
                    line: 2,
                })
                .collect(),
        }
    }

    fn write_at(path: &Path, text: &str, time: SystemTime) -> PathBuf {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
        set_time(path, time);
        path.to_path_buf()
    }

    fn set_time(path: &Path, time: SystemTime) {
        std::fs::File::options()
            .write(true)
            .open(path)
            .unwrap()
            .set_modified(time)
            .unwrap();
    }

    fn read(path: &Path) -> String {
        #[expect(clippy::disallowed_methods, reason = "the test's own text")]
        String::from_utf8(std::fs::read(path).unwrap()).unwrap()
    }

    /// The files a page packs (design D11 of `wasm-window`): a file entry, every file below a folder entry, by their
    /// paths in the output folder and sorted; a missing entry gives nothing.
    #[test]
    fn copied_files_of_files_and_folders() {
        let d = tempfile::tempdir().unwrap();
        let t = SystemTime::now();
        write_at(&d.path().join("GAME.CFG"), "cfg", t);
        write_at(&d.path().join("graphics/a.vga"), "a", t);
        write_at(&d.path().join("graphics/sub/b.vga"), "b", t);
        let build = project(d.path(), &["graphics", "GAME.CFG", "missing.dat"]);
        let names: Vec<String> = copied_files(&build).into_iter().map(|(n, _)| n).collect();
        assert_eq!(names, ["GAME.CFG", "graphics/a.vga", "graphics/sub/b.vga"]);
        assert!(copied_files(&project(d.path(), &[])).is_empty());
    }

    /// The time rule: a missing file is copied with the source's time, a newer source goes over the copy, a newer
    /// copy is kept, and equal times copy nothing.
    #[test]
    fn a_file_is_copied_when_missing_or_older() {
        let d = tempfile::tempdir().unwrap();
        let t0 = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let src = write_at(&d.path().join("GAME.CFG"), "source", t0);
        let b = project(d.path(), &["GAME.CFG"]);
        let copy = d.path().join("build/GAME.CFG");
        prepare(&b).unwrap();
        assert_eq!(read(&copy), "source", "missing: copied");
        assert_eq!(
            std::fs::metadata(&copy).unwrap().modified().unwrap(),
            t0,
            "with the source's time"
        );

        // Equal times: nothing is copied, even when the bytes differ.
        write_at(&copy, "same time", t0);
        prepare(&b).unwrap();
        assert_eq!(read(&copy), "same time");

        // The program rewrote its file: the copy is newer and stays.
        write_at(&copy, "the program's", t0 + Duration::from_secs(60));
        prepare(&b).unwrap();
        assert_eq!(read(&copy), "the program's");

        // The source edited after that: it goes over the copy.
        write_at(&src, "edited", t0 + Duration::from_secs(120));
        prepare(&b).unwrap();
        assert_eq!(read(&copy), "edited");
    }

    /// A folder is copied file by file into its own folders; a file deleted from the source stays in the copy.
    #[test]
    fn a_folder_is_copied_file_by_file() {
        let d = tempfile::tempdir().unwrap();
        let t0 = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        write_at(&d.path().join("maps/one.map"), "one", t0);
        let old = write_at(&d.path().join("maps/deep/old.map"), "old", t0);
        let b = project(d.path(), &["maps"]);
        prepare(&b).unwrap();
        assert_eq!(read(&d.path().join("build/maps/one.map")), "one");
        assert_eq!(read(&d.path().join("build/maps/deep/old.map")), "old");
        std::fs::remove_file(old).unwrap();
        write_at(
            &d.path().join("maps/one.map"),
            "one edited",
            t0 + Duration::from_secs(5),
        );
        prepare(&b).unwrap();
        assert_eq!(read(&d.path().join("build/maps/one.map")), "one edited");
        assert_eq!(
            read(&d.path().join("build/maps/deep/old.map")),
            "old",
            "a deleted file stays"
        );
    }

    /// A missing entry is an error at its line in the settings file, and nothing is made.
    #[test]
    fn a_missing_entry_is_an_error() {
        let d = tempfile::tempdir().unwrap();
        let e = prepare(&project(d.path(), &["GAME.CFG"])).unwrap_err();
        let file = d.path().join("qb64rust.toml");
        assert_eq!(
            e,
            format!(
                "{}:2: `copy` names \"GAME.CFG\", and there is no such file or folder",
                file.display()
            )
        );
        assert!(!d.path().join("build").exists());
    }

    /// No `copy` list: the output folder is made, and nothing else.
    #[test]
    fn the_output_folder_is_made() {
        let d = tempfile::tempdir().unwrap();
        prepare(&project(d.path(), &[])).unwrap();
        assert!(d.path().join("build").is_dir());
        assert_eq!(std::fs::read_dir(d.path().join("build")).unwrap().count(), 0);
    }
}
