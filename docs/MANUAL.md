# qb64rust: the compiler on its own

This download is the QB64Rust compiler without the VS Code extension. It compiles programs written for
QB64 Phoenix Edition 4.7.0 into executables from a command prompt, a build script or another editor:
`qb64rust-win-x64-<version>.zip` for Windows, `qb64rust-linux-x64-<version>.tar.gz` for Linux (see "On Linux"
below). QB64pe does not need to be installed. If you use VS Code, the extension (`qb64rust-win32-x64-<version>.vsix`
or `qb64rust-linux-x64-<version>.vsix` on the same release page) has this compiler inside it, so you don't need this
download as well.

## What you need

- Windows 10 version 1803 or later, 64-bit (x64); or Linux x64 with glibc (see "On Linux").
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

   If the download fails on this machine, download the same zip on another one and copy it over. Then run
   `qb64rust toolchain install --from-zip <zip>`, which checks it against the same checksum and unpacks it in the
   same way. The error message of a failed download names the zip's address.

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

## On Linux

The Linux download works the same way, with three differences. The C++ toolchain is your system's: `g++`, `make`
and `objcopy` on `PATH`, and the development packages libqb is built with (OpenGL, GLU, X11, ALSA). Nothing is
downloaded: `qb64rust toolchain install` checks that they are there and, when they are not, prints the command that
installs them for your distribution. An executable has no extension, as with QB64pe on Linux. And the debugger needs
`lldb-dap` 20 or later, from the package `lldb` (on Debian and Ubuntu a versioned one, `sudo apt-get install
lldb-20`: Ubuntu 24.04's plain `lldb` is 18, which has no data breakpoints, and 19's `lldb-dap` cannot show
values).

Install the packages with the line for your distribution (Ubuntu is the one tested):

```
sudo apt-get install build-essential libglu1-mesa-dev libasound2-dev libpng-dev libcurl4-openssl-dev libx11-dev libxcursor-dev libxrandr-dev libxinerama-dev libxi-dev
sudo dnf install gcc-c++ make mesa-libGLU-devel alsa-lib-devel libpng-devel libcurl-devel libX11-devel libXcursor-devel libXrandr-devel libXinerama-devel libXi-devel
sudo pacman -S gcc make zlib curl glu alsa-lib libx11 libxcursor libxrandr libxinerama libxi
```

The first is for Debian and Ubuntu (and Linux Mint), the second for Fedora, the third for Arch. Then unpack the
download into your home folder and compile:

```
tar -xzf qb64rust-linux-x64-<version>.tar.gz
cd qb64rust-<version>
./qb64rust -x hello.bas
./hello
```

`./qb64rust toolchain path` shows the folder of the `g++` a build would use and `system`. A toolchain folder of your
own (one with `bin/g++` and `bin/make`) can be given with `--toolchain-root <dir>` or `QB64RUST_TOOLCHAIN_ROOT`, as
on Windows. A program behaves as QB64pe's Linux build of it does: the console prints a comma's spacing as Linux's
QB64pe does, for instance, and `_OS$` is `[LINUX][64BIT]`.

## Options

`qb64rust --help` prints them all. The ones you are most likely to need:

| Option | What it does |
|---|---|
| `-o <file>` | The executable to write |
| `-x` | Accepted for QB64pe's command line; a build is always to an executable. `-c` is the same |
| `-e` | Build as if the program began with `OPTION _EXPLICIT`, its `$INCLUDE` files too |
| `-p` | Remove libqb's built objects first, so that the next build makes them again; alone, it does only that |
| `-w` | Print warnings too, the warnings about likely mistakes among them; see [Warnings about likely mistakes](#warnings-about-likely-mistakes) |
| `-q` | Print less while building |
| `-z` | Check the program and write its C++ without building an executable |
| `-y` | Lay the file out as QB64pe's IDE does; give the result with `-o <out.bas>` |
| `-f:<setting>=<value>` | A QB64pe compiler setting, such as `-f:OptimizeCppProgram=true`, `-f:StripDebugSymbols=false`, `-f:ExtraLinkerFlags=<flags>` or `-f:GenerateLicenseFile=true`; with `-y`, a layout setting such as `-f:AutoIndentSize=2` |
| `--debug` | A build for a debugger: with debug information, never optimised |
| `--coverage` | A build that counts how often each statement runs; see [Measuring which lines run](#measuring-which-lines-run) |
| `--trace-values` | A build that records the values each variable takes; see [Recording the values of variables](#recording-the-values-of-variables) |
| `--include-root <dir>` | Where `$INCLUDE` files and `DECLARE LIBRARY` headers are looked for after the program's own folder (by default the folder of `qb64rust.exe`, as QB64pe looks in its own folder) |
| `--toolchain-root <dir>` | The C++ toolchain to use instead of the installed one |
| `--libqb-root <dir>` | Another copy of libqb, for example one in a folder you can write to |
| `--version` | Print the compiler's version |

A program that uses `$INCLUDE` libraries meant to sit beside `qb64pe.exe` finds them beside `qb64rust.exe`.

## Warnings about likely mistakes

Some code compiles but is probably a mistake. The compiler warns about it. With `-w` the warnings are printed
with the others, and in the editor they show as warnings, with the warning's name beside the message. They never
stop a build and never change the program.

| Name | What it warns about | Example |
|---|---|---|
| `used-once` | A variable that no `DIM` or other declaration made, and whose name occurs only once in the whole program. It is often a misspelling | `total = 5` then `PRINT totl`: `` `totl` is used only once: is it misspelled? `` |
| `unused-procedure` | A SUB or FUNCTION that nothing calls. A `DECLARE` line is not a call | ``SUB `Spare` is never called`` |
| `unused-label` | A named label that no `GOTO`, `GOSUB`, `RESTORE`, `RESUME`, `ON ERROR` or event names. Line numbers are never reported | ``label `again` is never used`` |
| `hides-shared` | A `DIM` or `REDIM` in a SUB or FUNCTION that makes a new local variable with the name and type of a `DIM SHARED` one | ``this `score` is a new local variable, which hides the `DIM SHARED` one of the main module`` |
| `unreachable` | Statements after `GOTO`, `END`, `SYSTEM`, `STOP`, `EXIT`, `RETURN`, `RESUME` or `RUN`, with no label or line number before them. One warning for each such stretch | `this code is never run: no label follows the GOTO on line 8` |

Some cases are left out on purpose:
- `used-once`: a `FOR` counter, and a variable that `INPUT`, `LINE INPUT` or `READ` reads into, such as
  `INPUT "Press Enter", dummy$`. With `OPTION _EXPLICIT` (or `-e`) every variable is declared, so there is none.
- `unused-procedure`: `SUB _GL`, a routine of a `DECLARE LIBRARY` block, and a test SUB of a `_test.bas` file. A
  SUB named by `ON TIMER(1) Tick` counts as used. A procedure that only calls itself counts as used too.
- `unreachable`: `ERROR n`, `GOSUB`, `ON … GOTO` and `RUN file$` can go on with the next statement, so code after
  them is not reported. Neither is code after a single-line `IF … THEN GOTO`, nor code after a `$IF`, `$ELSE` or
  `$END IF` line, which may run on another platform.
- `used-once`, `unused-procedure` and `unused-label`: a name used in a `$IF` branch that this compile skips. A
  `$IF WIN THEN` branch that calls `WinSetup` counts as a call on Linux too.

These warnings are given only for the file you compile, not for the files it includes with `$INCLUDE`. A
library file usually has routines a program does not use.

`-w` also gives the warnings QB64pe's `-w` gives, in its words and at its lines, included files too:

| Name | What it warns about |
|---|---|
| `unused-variable` | "Unused variable": a variable made by `DIM`, `REDIM`, `STATIC` or a parameter that is never named again, or a FUNCTION's result the FUNCTION never sets. The next line gives the name and type, as `dimonly%   INTEGER` |
| `empty-select` | "Empty SELECT CASE block": a `SELECT CASE` with no `CASE`, at its `END SELECT` |
| `duplicate-const` | "Duplicate constant definition": a `CONST` repeated with the same value. The next line gives it, as `a = 5` |
| `debug-incompatible` | "Feature incompatible with $DEBUG mode": `CHAIN` or `RUN` in a program with `$DEBUG` |

A variable that no `DIM` made gets `used-once` when it is named once, never "Unused variable". Our lines have a
column after the line number, and each warning is printed once, in source order. QB64pe prints its unused
variables last.

The warnings are given only when the program has no errors. Fix the errors first.

`qb64rust --report warnings <file or folder>...` counts the warnings over many programs, by name, and then prints
each one. It builds nothing.

## Testing your code

`qb64rust test` runs unit tests written in BASIC. A test file is a file whose name ends in `_test.bas`; it
includes the code it tests (`'$INCLUDE:'math.bm'`), and each `SUB` in it whose name starts with `Test` and that
takes no parameters is a test:

```
'$INCLUDE:'math.bm'

SUB TestAdd
    AssertEqualInt 5, AddUp&(2, 3), "sum"
END SUB
```

```
qb64rust test
qb64rust test lib\math_test.bas --filter TestAdd
```

With no file or folder named, it runs every test file in the current folder and below it. Each test runs in a
program of its own, and passes only when its SUB returns: a failed assertion is a failure, and a runtime error,
`END` or `SYSTEM`, or more than 30 seconds (`--timeout <seconds>`) is an error, whatever the test printed. The
assertions are `AssertTrue`, `AssertFalse`, `AssertEqualInt`, `AssertEqualSingle`, `AssertEqualDouble`,
`AssertNear`, `AssertEqualStr` and `Fail`; `_ASSERT` works too, and `TestTempDir$` gives each test an empty folder of
its own. Tests are built as `$CONSOLE:ONLY` programs, so they are for code that does not draw on the screen
(images made with `_NEWIMAGE` work). `--list` lists the tests without running them, and `--format json` gives one
JSON object per line for other tools. `--coverage` counts the statements the tests run (see
[Measuring which lines run](#measuring-which-lines-run)). The exit status is 0 when every test passed, and 1 otherwise.

## Measuring which lines run

`qb64rust --coverage` builds a program that counts how many times each statement runs. Set the environment
variable `QB64RUST_COVERAGE` to a file name before running it:

```
qb64rust game.bas --coverage
set QB64RUST_COVERAGE=counts.txt
game.exe
```

When the program ends, it adds one record to that file. The program can end by `END` or `SYSTEM`, by running off
its end, by a runtime error, or by its window being closed. The record lists every statement with its file, line,
column and count, so statements that never ran are listed with 0. It also names the built-ins each statement
calls. Several runs can write to the same file; each adds its own record. Without `QB64RUST_COVERAGE`, the
program writes nothing and behaves as a normal build.

Unit tests can be counted too. `qb64rust test` with `--coverage` builds each test program this way, and adds one
record per test to the file `QB64RUST_COVERAGE` names:

```
set QB64RUST_COVERAGE=counts.txt
qb64rust test tests --coverage
```

A test's record also names the test's entry point (`<name>_test.bas.test-entry.bas`) and the test library
(`qb64rust-test/qbtest.bi` and `qbtest.bm`). The runner makes these files in memory, so they are not on disk.
Files included from another folder are named by their path from the test file's folder, such as `../game.bm`.

`tools\coverage\coverage.py` in the repository merges the records and lists the lines that did not run. The
record format is described in [tools/coverage/README.md](../tools/coverage/README.md).

## Recording the values of variables

`qb64rust --trace-values` builds a program that records the value each store leaves in a variable. Use it to
see which values a variable really takes over a whole run: its range, or that it never leaves a few values. Set
the environment variable `QB64RUST_TRACE` to a file name before running the program:

```
qb64rust game.bas --trace-values
set QB64RUST_TRACE=values.txt
game.exe
python tools\trace\value_trace.py values.txt
```

When the program ends, it adds one record to that file. The program can end by `END` or `SYSTEM`, by running off
its end, by a runtime error, or by its window being closed. Several runs can write to the same file; each adds
its own record. Without `QB64RUST_TRACE`, the program writes nothing.

`tools\trace\value_trace.py` in the repository merges the records and prints one block per variable:

```
HP  INTEGER  in MOVESHIP
  412 store(s), 0 after-call record(s), range 0 to 100, 23 distinct value(s)
       120 x  100    first at game.bas:88
        61 x  95    first at game.bas:140
```

What is recorded:

- A scalar variable, an array (all its elements together) and a member of a user type (`ship.hp`) each have an
  entry. A variable of the same name in another `SUB` or `FUNCTION` has its own.
- Each entry gives the number of stores, the distinct values with their counts and the line that first stored
  each, and the smallest and largest value. For a string it gives the shortest and longest length.
- An entry keeps at most 256 distinct values. After that it still counts stores and widens the range, and the
  report marks it as cut. Set `QB64RUST_TRACE_LIMIT` to a number for another limit.
- The value a variable starts with (0 or an empty string) is not a store and is not listed.
- A variable passed to a `SUB` or `FUNCTION` is read again after the call, and its value is recorded when it
  changed. An array passed to one is not read again: the report names the parameter whose entry holds the values
  stored through it.
- A statement that fills an array with new values (`b() = a()`, `_ARRAYCOPY`, `GET #1, , a()`) is recorded
  element by element. The strings of a `FIELD` are read again after each `GET #` into the record buffer.
- `REDIM` and `ERASE` write zeros, and statements such as `POKE`, `_MEMPUT` and `BLOAD` write at an address.
  They are counted and listed, without values.

A traced program behaves as the normal build does, with the same output, errors and exit code. It runs slower
and uses more memory, so build without the option for everyday use. The option combines with `--coverage` and
`--debug`. The trace tool's own options (JSON output, one variable by name), the full list of what is not
recorded, and the record format are described in [tools/trace/README.md](../tools/trace/README.md).

## Where it differs from QB64pe

The compiler takes the QB64pe 4.7.0 dialect and its programs behave as QB64pe's do, with a short list of
deliberate exceptions (most of them cases where QB64pe's own result is undefined, or where a program would crash).
They are listed in
[`QB64PE-DIVERGENCES.md`](https://github.com/davidshq/QB64Rust/blob/main/docs/QB64PE-DIVERGENCES.md). A few of QB64pe's features
are not supported yet. A program that uses one gets the error "not supported yet", which names the feature.

The one a script is most likely to notice: **a program that ends because of a runtime error exits with code 1**,
where QB64pe's exits with 0. That is an error no `ON ERROR` handler takes, when the program ends at it (with
`QB64PE_NOPROMPT=y`, or "no" at the "Continue?" dialog), or a critical error such as a failed `_ASSERT`. A program
that goes on after the error, and a program's own `END n` or `SYSTEM n`, keep their code.

## Reporting a problem

Open an issue at [github.com/davidshq/QB64Rust/issues](https://github.com/davidshq/QB64Rust/issues). Include
what `qb64rust --version` prints, the command you ran, its output, and the program (or the smallest part of it
that shows the problem).
