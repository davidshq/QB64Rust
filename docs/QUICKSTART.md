# Quick start

This guide takes you from nothing installed to a QB64 program that builds, runs and stops at a breakpoint. It
takes about ten minutes, most of it one download.

QB64Rust compiles programs written for QB64 Phoenix Edition 4.7.0. You do not need QB64pe installed.

## 1. Choose a download

There are two downloads. Both are on the [releases](https://github.com/davidshq/QB64Rust/releases) page.

| You want | Download | Then go to |
|---|---|---|
| An editor, compiler and debugger in VS Code | `qb64rust-win32-x64-<version>.vsix` (Windows) or `qb64rust-linux-x64-<version>.vsix` (Linux) | Section 2 |
| Only the compiler, for a command prompt or another editor | `qb64rust-win-x64-<version>.zip` (Windows) or `qb64rust-linux-x64-<version>.tar.gz` (Linux) | Section 3 |

Most people want the first. The VS Code extension has the compiler inside it, so you never need both.

What your machine needs:

- Windows 10 version 1803 or later, 64-bit, or Linux x64 with glibc. Ubuntu 24.04 is the Linux that is tested.
  There is no macOS release.
- About 500 MB of free disk space.
- For the extension, VS Code 1.100 or later.

## 2. VS Code

### Install the extension

1. Open VS Code.
2. Open the Command Palette (`Ctrl+Shift+P`) and run **Extensions: Install from VSIX…**.
3. Pick the `.vsix` file you downloaded.

Or run this in a terminal:

```
code --install-extension qb64rust-win32-x64-<version>.vsix
```

**On Linux, also install the build packages.** Builds use your system's `g++`, and the debugger needs `lldb-dap`
20 or later. On Ubuntu and Debian:

```
sudo apt-get install build-essential libglu1-mesa-dev libasound2-dev libpng-dev libcurl4-openssl-dev libx11-dev libxcursor-dev libxrandr-dev libxinerama-dev libxi-dev
sudo apt-get install lldb-20
```

The lines for Fedora and Arch are in [MANUAL.md](MANUAL.md#on-linux).

### Write your first program

The walkthrough **Get Started with QB64**, in VS Code's Welcome page, takes you through these steps with a button
for each. **QB64: Get Started** opens it at any time. Without it:

1. Open the Command Palette and run **QB64: New Program**.
2. A save dialog asks for the folder and the name, as for any new file. Choose **Create Program**.

The command writes that one file, which holds `PRINT "Hello, world!"`, and opens it. It writes no other file.

For a program of several files, or one with every fix of a questionable behavior kept from QB64pe turned on, run
**QB64: New Project** instead and pick an empty folder. It writes `main.bas` and `qb64rust.toml`, the program's
settings file, which turns the fixes on. `qb64rust --list-fixes` lists them.

You can also open any `.bas` file you already have. An existing program needs no settings file.

### Build and run it

Click **Run** in the QB64 side panel (the QB64 icon in the activity bar) or at the left of the status bar. **QB64:
Build and Run** in the Command Palette and the run button at the top right of the editor do the same.

**On Windows, the first build asks to download the C++ toolchain.** Choose **Download**. It is llvm-mingw, a
190 MB download that takes about 460 MB on disk. It is fetched once. On Linux nothing is downloaded. If a package
is missing, the build says which one and offers a button that copies the install command.

**The first build takes about 40 seconds**, because it builds the runtime library once. Later builds reuse it and
are much faster.

The executable is written into a folder `build` next to the source: `main.bas` becomes `build\main.exe`
(`build/main` on Linux). It runs in the source's folder, so it finds the files beside `main.bas`. The compiler's
output is in the "QB64" output channel.

### Fix an error

Add the line `PRINT (2 + 3` (no closing parenthesis). A red underline appears as you type, the error is listed in
the Problems panel, and the QB64 side panel's Program view shows it with a link to its line. (A string with no
closing quote is no error: QB64 ends it at the end of the line, as QBasic does.) Syntax errors show as you type.
All other errors, such as a wrong type or a SUB that is not defined, show when you save.

### Debug it

1. Click in the gutter to the left of a line number to set a breakpoint.
2. Press **F5**.

The program is built for debugging and stops at your line. You can step by BASIC line, see variables under
"Locals" and "Main module", and hover a name to see its value.

### What else the editor does

| To do this | Use |
|---|---|
| Lay the code out as QB64pe's IDE does | **Format Document** |
| Jump to a SUB, FUNCTION, label or variable | **Go to Definition** (`F12`) |
| Rename a name everywhere in the program | **Rename Symbol** (`F2`) |
| See a name's type or a SUB's header | Hover over it |
| Read the help page of a statement or function | **QB64: Help for This Word** (right-click it), or the Help view |
| Try a program that is already written | The Examples view of the QB64 side panel |
| Run unit tests written in BASIC | The Test Explorer (the beaker icon) |

The full description of each is in [vscode/README.md](../vscode/README.md).

## 3. The compiler on its own

### Install it on Windows

1. Unpack the zip into a folder you can write to, such as `C:\qb64rust`. Do not use a folder under
   `Program Files`: the compiler builds its runtime library inside its own folder.
2. Open a command prompt in the unpacked folder `qb64rust-<version>` and run:

   ```
   qb64rust toolchain install
   ```

   This downloads the C++ toolchain (llvm-mingw, 190 MB) into `%LOCALAPPDATA%\qb64rust\toolchain`. It is the only
   thing the compiler ever downloads.

### Install it on Linux

Install the build packages (the `apt-get` line with `build-essential` in section 2, or the Fedora or Arch line in
[MANUAL.md](MANUAL.md#on-linux)). Then unpack the download:

```
tar -xzf qb64rust-linux-x64-<version>.tar.gz
cd qb64rust-<version>
./qb64rust toolchain install
```

On Linux `toolchain install` downloads nothing. It checks that the packages are there and names any that are
missing.

### Compile and run a program

Save this as `hello.bas`:

```basic
PRINT "Hello, world!"
```

Then, on Windows:

```
qb64rust -x hello.bas
build\hello.exe
```

On Linux:

```
./qb64rust -x hello.bas
./build/hello
```

The executable is named after the program and put in a folder `build` beside it. Use `-o <file>` to name it
yourself. The first build
takes up to a minute while the runtime library is built.

Errors are printed as `file:line:column: error: message`. The exit status is 0 when the program compiled and 1
when it has errors.

### Commands worth knowing

| Command | What it does |
|---|---|
| `qb64rust -x game.bas -o game.exe` | Build an executable |
| `qb64rust -z game.bas` | Check the program without building it |
| `qb64rust -w -x game.bas` | Build, and print warnings about likely mistakes |
| `qb64rust -y game.bas -o out.bas` | Lay the code out as QB64pe's IDE does |
| `qb64rust test` | Run the unit tests in the current folder and below |
| `qb64rust --help` | List every option |

## 4. Next steps

- **Bring an existing QB64pe program.** Open or compile it as it is. The compiler takes the QB64pe 4.7.0 dialect
  and QB64pe's command-line options. A program that uses a feature not built yet gets the error
  "not supported yet", which names the feature.
- **Write unit tests.** A file whose name ends in `_test.bas` is a test file. Each `SUB` in it whose name starts
  with `Test` is a test. See [MANUAL.md](MANUAL.md#testing-your-code).
- **Play a game in a browser.** This is experimental. In VS Code, run **QB64: Run in Browser** on a window
  program. At a prompt, `qb64rust -x --target wasm game.bas` and then `qb64rust serve game.bas --open`. The
  first time, the Emscripten SDK is downloaded (815 MB). See
  [MANUAL.md](MANUAL.md#a-game-in-a-browser).
- **Read the manual.** [MANUAL.md](MANUAL.md) covers every option, the settings file, warnings, coverage builds
  and value traces.
- **Look a word up.** The [language reference](reference/README.md) has a page for every statement, function and
  metacommand, with what it does, its errors and an example.
- **Check the differences.** The few places where a program behaves differently than with QB64pe are listed in
  [DIVERGENCES-QB64PE.md](DIVERGENCES-QB64PE.md).
- **Report a problem.** Open an issue at
  [github.com/davidshq/QB64Rust/issues](https://github.com/davidshq/QB64Rust/issues). Include what
  `qb64rust --version` prints, the command you ran, its output and the program.
