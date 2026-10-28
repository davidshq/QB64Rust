# qb64rust: the compiler on its own

This download is the QB64Rust compiler without the VS Code extension. It compiles programs written for
QB64 Phoenix Edition 4.7.0 into Windows executables from a command prompt, a build script or another editor.
QB64pe does not need to be installed. If you use VS Code, the extension (`qb64rust-win32-x64-<version>.vsix` on
the same release page) has this compiler inside it, so you don't need this download as well.

## What you need

- Windows 10 version 1803 or later, 64-bit (x64).
- About 500 MB of disk space: 460 MB for the C++ toolchain, the rest for the compiler, its copy of libqb and the
  objects it builds there.
- A network connection once, to fetch the toolchain.

## Setting it up

1. Unpack the zip into a folder **you can write to**, such as a folder in your Documents or `C:\qb64rust`. The
   compiler builds libqb's objects inside its own `libqb` folder the first time you compile, so a folder under
   `Program Files` does not work. In such a folder the first build stops with a message that says so.
2. Open a command prompt in the unpacked folder and run:

   ```
   qb64rust toolchain install
   ```

   This downloads the C++ toolchain the compiler uses (llvm-mingw, a 190 MB download from
   `github.com/mstorsjo/llvm-mingw`, checked against a fixed checksum) and unpacks it into
   `%LOCALAPPDATA%\qb64rust\toolchain`. It is the only thing the compiler downloads, and it downloads it only when
   you run this command. Run it again at any time: when the toolchain is already there, it does nothing.
   `qb64rust toolchain path` shows the toolchain a build would use.

If you already have a toolchain folder you want to use instead, give it to each build with
`--toolchain-root <dir>`, or set the environment variable `QB64RUST_TOOLCHAIN_ROOT`.

## Compiling and running a program

```
qb64rust -x hello.bas -o hello.exe
hello.exe
```

Without `-o`, the executable is named after the program and put beside it. The first build takes longer, up to a
minute, while libqb's objects are built. Later builds reuse them.

Errors and warnings are printed as `file:line:column: error: message`. The exit status is:

| Status | Meaning |
|---|---|
| 0 | The program compiled (and the executable was built) |
| 1 | The program has errors, or something the build needs is missing; no executable is left behind |
| 3 | An internal error of the compiler; please report it (see the end of this page) |

## Options

`qb64rust --help` prints them all. The ones you are most likely to need:

| Option | What it does |
|---|---|
| `-o <file>` | The executable to write |
| `-x` | Accepted for QB64pe's command line; a build is always to an executable |
| `-w` | Print warnings too |
| `-q` | Print less while building |
| `-z` | Check the program and write its C++ without building an executable |
| `-y` | Lay the file out as QB64pe's IDE does; give the result with `-o <out.bas>` |
| `-f:<setting>=<value>` | A QB64pe compiler setting, such as `-f:OptimizeCppProgram=true`, `-f:StripDebugSymbols=false`, `-f:ExtraLinkerFlags=<flags>` or `-f:GenerateLicenseFile=true`; with `-y`, a layout setting such as `-f:AutoIndentSize=2` |
| `--debug` | A build for a debugger: with debug information, never optimised |
| `--include-root <dir>` | Where `$INCLUDE` files and `DECLARE LIBRARY` headers are looked for after the program's own folder (by default the folder of `qb64rust.exe`, as QB64pe looks in its own folder) |
| `--toolchain-root <dir>` | The C++ toolchain to use instead of the installed one |
| `--libqb-root <dir>` | Another copy of libqb, for example one in a folder you can write to |
| `--version` | Print the compiler's version |

A program that uses `$INCLUDE` libraries meant to sit beside `qb64pe.exe` finds them beside `qb64rust.exe`.

## Where it differs from QB64pe

The compiler takes the QB64pe 4.7.0 dialect and its programs behave as QB64pe's do, with a short list of
deliberate exceptions (most of them cases where QB64pe's own result is undefined, or where a program would crash).
They are listed in
[`QB64PE-DIVERGENCES.md`](https://github.com/davidshq/QB64Rust/blob/main/QB64PE-DIVERGENCES.md). A few of QB64pe's features
are not supported yet. A program that uses one gets the error "not supported yet", which names the feature.

## Reporting a problem

Open an issue at [github.com/davidshq/QB64Rust/issues](https://github.com/davidshq/QB64Rust/issues). Include
what `qb64rust --version` prints, the command you ran, its output, and the program (or the smallest part of it
that shows the problem).
