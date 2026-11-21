## MODIFIED Requirements

### Requirement: qb64pe-compatible compile
`qb64rust [-x] [-q] [-m] [-w] [-f:<setting>=<value>]... <file.bas> [-o <exe>]` SHALL compile the file to the
executable (default: the source name with `.exe` on Windows and with no extension on Linux, as `qb64pe` names it
on each, in the output folder: requirement "The output folder"). `-x`, `-q`, `-m` and `-w` SHALL be accepted; `-c` SHALL be accepted as another
name for `-x`, since there is no IDE window to build in. `-s` and `-s:<setting>[=<value>]`, `qb64pe`'s stored
settings, SHALL be an error that names `-f:<setting>=<value>`. `-e` SHALL check the program as if it began with
`OPTION _EXPLICIT`, as `qb64pe -e` does: its included files too, `_EXPLICIT_` and `_EXPLICITARRAY_` set from the
first round, an `OPTION _EXPLICIT` of the program's own no error, and the same with `-z` and `-y`. `-p` SHALL first
remove the libqb objects that builds wrote in the copy of libqb, and its stamp, holding the objects' lock alone,
as `qb64pe -p` runs libqb's `make clean`; then the rest of the command line SHALL run, and `-p` with no input file
SHALL end there with exit status 0. `-p` SHALL refuse a QB64pe folder (one with `source/qb64pe.bas`, such as the
reference clone), removing nothing;
`-q` SHALL suppress progress output. `-f:OptimizeCppProgram=true|false` SHALL build the C++ with or without
optimisation as `qb64pe` does; `-f:ExtraLinkerFlags=<value>` SHALL add the value to the end of the link command as
`qb64pe` does; `-f:StripDebugSymbols=true|false` SHALL strip or not strip the executable's symbols as `qb64pe`
does (true, the default, strips; any other value is an error as for the other boolean settings);
`-f:GenerateLicenseFile=true|false` SHALL, when true, write the licence file `<exe>.license.txt` beside the
executable as `qb64pe` does, naming the licences of the parts of libqb the program links; the layout settings of
spec `compiler/format` SHALL be checked as `-y` checks them and SHALL NOT change the build, as with `qb64pe`; any
other setting SHALL be an error "setting not supported yet". Exit status SHALL be 0 when the executable was built
and 1 otherwise; on failure no executable SHALL be left at the output path.

#### Scenario: Runner's compile line
- **WHEN** the corpus runner runs `qb64rust -q -m -x <prog>.bas -o <prog>.exe` on a slice program
- **THEN** `<prog>.exe` exists and the exit status is 0

#### Scenario: qb64pe's -c and -s
- **WHEN** `qb64rust -c p.bas` runs on a program that compiles
- **THEN** it builds `p.exe` as `qb64rust -x p.bas` does
- **WHEN** `qb64rust -s:debuginfo=true p.bas` runs
- **THEN** the exit status is 1, nothing is built, and the message names `-f:<setting>=<value>`

#### Scenario: qb64pe's -e
- **WHEN** `qb64rust -e -z p.bas` runs on a program whose line 1 is `x = 1`, with no `DIM`
- **THEN** the exit status is 1 and the error at `p.bas:1:1` is that `x` is not declared, with the note
  "QB64pe: Variable 'x' (SINGLE) not defined"
- **WHEN** the same program is compiled without `-e`
- **THEN** the exit status is 0

#### Scenario: qb64pe's -p
- **WHEN** `qb64rust -p --libqb-root <copy>` runs
- **THEN** the copy's `.o`, `.a` and `.tmp` files under `internal/c` and its `objects.stamp` are gone, its sources
  stay, and the exit status is 0
- **WHEN** `qb64rust -p -z p.bas` runs
- **THEN** the objects are removed and then `p.bas` is compiled as without `-p`
- **WHEN** `qb64rust -p --libqb-root <qb64pe>` runs on a folder with `source/qb64pe.bas`
- **THEN** the exit status is 1, the message says it is a QB64pe folder, and nothing is removed

#### Scenario: Program with an error
- **WHEN** the source has an error
- **THEN** the exit status is 1, the diagnostics are printed, and `<prog>.exe` does not exist

#### Scenario: Compile suite's settings
- **WHEN** the upstream compile suite runs `qb64rust -f:OptimizeCppProgram=true -f:StripDebugSymbols=false -q -m -x
  <prog>.bas -o <exe>` on a program in `tests/upstream/pass.list`
- **THEN** the executable is built unstripped, prints what it printed before, and the exit status is 0

#### Scenario: Unstripped on request
- **WHEN** a program is built with `-f:StripDebugSymbols=false` and once without the setting
- **THEN** the first executable keeps its local symbols (`nm` lists symbols of type `t`) and the second has none;
  both keep their global ones, as the `Makefile`'s `objcopy --strip-unneeded` leaves them (measured at task 1.3)

#### Scenario: Unknown setting
- **WHEN** `-f:MaxCompilerProcesses=2` is given
- **THEN** the exit status is 1 with the message that the setting is not supported yet, and no executable exists

#### Scenario: A layout setting in a build
- **WHEN** a program is built with `-f:AutoIndent=false`
- **THEN** the executable is built as without the setting, and the exit status is 0

#### Scenario: The licence file
- **WHEN** `-f:GenerateLicenseFile=true` is given for a program that names `_INFLATE$`
- **THEN** `<exe>.license.txt` exists and equals, but for line ends, the file the old compiler writes for it

#### Scenario: No licence file unless asked
- **WHEN** a program is built without the setting, or with `-f:GenerateLicenseFile=false`
- **THEN** no `<exe>.license.txt` is written, and one an earlier build left at that path stays, as with the old
  compiler

#### Scenario: An extra source file on the link line
- **WHEN** `-f:ExtraLinkerFlags=<path>/screen_state.cpp` is given and the program runs with `QB64_SCREEN_STATE`
  naming a file
- **THEN** the executable is built with that file linked in, and the run writes the state file

#### Scenario: The extra file changes
- **WHEN** a build cache is in use and the file named by `-f:ExtraLinkerFlags` has changed since the cached build
- **THEN** the program is built again

#### Scenario: Default name on Linux
- **WHEN** `qb64rust -x game.bas` runs on Linux with no `-o`
- **THEN** the executable is `build/game`, with its execute permission set

### Requirement: The output of a wasm build
With `--target wasm` the default output SHALL be the source's name with `.js`, in the output folder
(requirement "The output folder"), and the `.wasm` file SHALL have the same name beside it. `-o <file>` SHALL
name the `.js` file and SHALL end in `.js`; another ending is an error with status 1. On failure neither file SHALL be left at the output path.

#### Scenario: A named output
- **WHEN** `qb64rust -x --target wasm p.bas -o out/app.js` builds
- **THEN** `out/app.js` and `out/app.wasm` exist

#### Scenario: A wrong ending
- **WHEN** `qb64rust -x --target wasm p.bas -o p.exe` runs
- **THEN** the exit status is 1 and the message says the output of a wasm build ends in `.js`

#### Scenario: A failed build
- **WHEN** the C++ build of a wasm program fails and `p.js` and `p.wasm` of an earlier build exist
- **THEN** both are gone after the command, as an executable is after a failed build

## ADDED Requirements

### Requirement: The output folder
When `-x` (or `-c`) has no `-o`, the program SHALL be written under its default name into the output folder:
`[build]` `output` of the program's settings file when it is set, else `build`. The folder SHALL be relative to
the settings file's folder, or to the source's folder when there is no settings file or `--no-settings` is
given. It SHALL be created when missing. `-o` SHALL win over the output folder.

#### Scenario: The default
- **WHEN** `game.bas` has no settings file and `qb64rust -x game.bas` runs on Windows
- **THEN** `build/game.exe` exists in `game.bas`'s folder, and no `game.exe` is written beside `game.bas`

#### Scenario: A settings file one folder up
- **WHEN** `proj/qb64rust.toml` has no `[build]` table and `qb64rust -x proj/src/game.bas` runs
- **THEN** the program is `proj/build/game.exe`

#### Scenario: Another folder
- **WHEN** `proj/qb64rust.toml` has `[build]` and `output = "out"`, and `qb64rust -x proj/src/game.bas` runs
- **THEN** the program is `proj/out/game.exe`

#### Scenario: Next to the settings file
- **WHEN** `qb64rust.toml` beside `game.bas` has `output = "."`
- **THEN** the program is `game.exe` beside `game.bas`, as before this change

#### Scenario: -o wins
- **WHEN** `qb64rust -x game.bas -o other/g.exe` runs
- **THEN** the program is `other/g.exe`, and no `build` folder is made

#### Scenario: A wasm build
- **WHEN** `qb64rust -x --target wasm p.bas` builds with no settings file
- **THEN** `build/p.js` and `build/p.wasm` exist

### Requirement: Files copied into the output folder
`[build]` `copy` SHALL list files and folders, relative to the settings file's folder, that a build without `-o`
copies into the output folder before it compiles. A file SHALL be copied when the output folder has none of that
name, or when the source's is newer; a folder SHALL be copied file by file by the same rule. Nothing in the output
folder SHALL be deleted. A build with `-o` SHALL copy nothing.

#### Scenario: The first build
- **WHEN** the settings file has `copy = ["GAME.CFG", "maps"]`, and `qb64rust -x game.bas` runs for the first
  time
- **THEN** `build/GAME.CFG` and every file of `maps`, in its own folders, are in `build`

#### Scenario: A file the program rewrote
- **WHEN** the program, run in `build`, has rewritten `build/GAME.CFG`, and the program is built again with
  `GAME.CFG` unchanged
- **THEN** `build/GAME.CFG` is the program's, not the source folder's

#### Scenario: A data file edited in the source folder
- **WHEN** `maps/one.map` is edited after a build, and the program is built again
- **THEN** `build/maps/one.map` is the edited file

#### Scenario: A file removed from the source folder
- **WHEN** `maps/old.map` is deleted after a build, and the program is built again
- **THEN** `build/maps/old.map` is still there

#### Scenario: A missing entry
- **WHEN** `copy` names `GAME.CFG` and there is no such file or folder
- **THEN** the exit status is 1, the message names the settings file, its line and `GAME.CFG`, and nothing is
  built

### Requirement: The folder a program runs in
The folder a program runs in SHALL be the output folder when the settings file's `copy` list has an entry, and
the source's folder otherwise. The compiler SHALL report it through `--print-paths`; the extension's Run and
Debug use it (specs `editor/build-run` and `editor/debug-launch`).

#### Scenario: No copy list
- **WHEN** `game.bas` has no settings file
- **THEN** the folder it runs in is `game.bas`'s folder, so it opens its data files where they are

#### Scenario: A copy list
- **WHEN** the settings file has `copy = ["GAME.CFG"]`
- **THEN** the folder it runs in is `build`, so a file it writes by name alone stays in `build`

### Requirement: A bad build table
A `[build]` table SHALL be an error that stops the build, as a mistake in any other table does, when it has a
key other than `output` and `copy`, when `output` is not a string or is empty, when `copy` is not a list of
strings, or when a path has a `\`. A `copy` entry SHALL NOT be or hold the output folder, so `copy` SHALL be an
error with `output = "."`. A `copy` entry SHALL be a path inside the settings file's folder: an absolute path or
one that goes above the folder with `..` is an error, since its copy would be outside the output folder.

#### Scenario: An unknown key
- **WHEN** `[build]` has `out = "build"`
- **THEN** the exit status is 1, and the message names the line and says the keys are `output` and `copy`

#### Scenario: The output folder inside a copied folder
- **WHEN** `output = "data/build"` and `copy = ["data"]`
- **THEN** the exit status is 1, and the message says that `data` holds the output folder

#### Scenario: The output folder reached another way
- **WHEN** `proj/qb64rust.toml` has `output = "../proj/data/out"`, or the same folder as an absolute path, and
  `copy = ["data"]`
- **THEN** the exit status is 1, and the message says that `data` holds the output folder: the paths are compared
  once joined to the settings file's folder

#### Scenario: A copy list with the settings file's folder as the output
- **WHEN** `output = "."` and `copy = ["GAME.CFG"]`
- **THEN** the exit status is 1, and the message says that the files are already in the output folder

### Requirement: The paths of a program
`qb64rust --print-paths <file.bas>` SHALL print one line of JSON with `exe`, the absolute path that `-x` with no
`-o` would write, and `run_in`, the absolute folder the program runs in, after reading the settings file. It
SHALL take `--target` and `--no-settings` as a build does, build and copy nothing, and exit with status 0. A
settings file that cannot be used SHALL be reported as a build reports it, with status 1.

#### Scenario: Without a settings file
- **WHEN** `<dir>/game.bas` has no settings file and `qb64rust --print-paths <dir>/game.bas` runs on Windows
- **THEN** it prints `{"exe":"<dir>\\build\\game.exe","run_in":"<dir>"}` and nothing is built

#### Scenario: With a copy list
- **WHEN** the settings file beside `<dir>/game.bas` has `copy = ["GAME.CFG"]`
- **THEN** it prints `{"exe":"<dir>\\build\\game.exe","run_in":"<dir>\\build"}`
