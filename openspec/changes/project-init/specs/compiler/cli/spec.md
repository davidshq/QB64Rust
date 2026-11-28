# Spec Delta

## MODIFIED Requirements

### Requirement: The output folder
When `-x` (or `-c`) has no `-o`, the program SHALL be written under its default name into the output folder. The
output folder SHALL be `[build]` `output` of the program's settings file when it is set. Otherwise it SHALL be
`build` when the settings file has a `copy` list or the target is wasm, and the source's folder in every other
case. `output` and `build` SHALL be relative to the settings file's folder, or to the source's folder when there is
no settings file or `--no-settings` is given. The folder SHALL be created when missing. `-o` SHALL win over the
output folder. When the program's path is a folder, the build SHALL stop before the compile with status 1 and a
message that names the folder and says how to choose another path.

#### Scenario: The default
- **WHEN** `game.bas` has no settings file and `qb64rust -x game.bas` runs on Windows
- **THEN** `game.exe` exists beside `game.bas`, and no `build` folder is made

#### Scenario: A settings file one folder up
- **WHEN** `proj/qb64rust.toml` has no `[build]` table and `qb64rust -x proj/src/game.bas` runs
- **THEN** the program is `proj/src/game.exe`

#### Scenario: A build table without an output folder
- **WHEN** `proj/qb64rust.toml` has `[build]` and `call_trace = true` only, and `qb64rust -x proj/src/game.bas` runs
- **THEN** the program is `proj/src/game.exe`

#### Scenario: Another folder
- **WHEN** `proj/qb64rust.toml` has `[build]` and `output = "out"`, and `qb64rust -x proj/src/game.bas` runs
- **THEN** the program is `proj/out/game.exe`

#### Scenario: A copy list without an output folder
- **WHEN** `proj/qb64rust.toml` has `copy = ["GAME.CFG"]` and no `output`, and `qb64rust -x proj/game.bas` runs
- **THEN** the program is `proj/build/game.exe`

#### Scenario: Next to the settings file
- **WHEN** `proj/qb64rust.toml` has `output = "."`, and `qb64rust -x proj/src/game.bas` runs
- **THEN** the program is `proj/game.exe`

#### Scenario: -o wins
- **WHEN** `qb64rust -x game.bas -o other/g.exe` runs
- **THEN** the program is `other/g.exe`, and no `game.exe` is written beside `game.bas`

#### Scenario: A wasm build
- **WHEN** `qb64rust -x --target wasm p.bas` builds with no settings file
- **THEN** `build/p.js` and `build/p.wasm` exist, and neither is written beside `p.bas`

#### Scenario: A folder of the program's name
- **WHEN** on Linux `maps.bas` has a folder `maps` beside it and `qb64rust -x maps.bas` runs
- **THEN** the status is 1, the message names the folder `maps` and says to use `-o` or `[build]` `output`, and
  nothing in `maps` changes

### Requirement: The folder a program runs in
The folder a program runs in SHALL be the folder the program is in: libqb makes the program's own folder the
current one when it starts, however it was started. The compiler SHALL report that folder through `--print-paths`;
the extension's Run and Debug start the program there (specs `editor/build-run` and `editor/debug-launch`). A build
SHALL add no code that changes the folder, and no environment variable SHALL change it.

#### Scenario: Started from the extension
- **WHEN** `game.exe` was built beside `game.bas` without a settings file and the extension's Run starts it
- **THEN** a file it writes by name alone goes beside `game.bas`

#### Scenario: Started any other way
- **WHEN** the same `game.exe` is started by a double click, or from a shell in another folder
- **THEN** it runs in `game.bas`'s folder too, as a QB64pe program runs in its own folder

#### Scenario: No copy list
- **WHEN** `game.bas` has no settings file
- **THEN** the folder it runs in is `game.bas`'s folder, so it opens its data files where they are

#### Scenario: A copy list
- **WHEN** the settings file has `copy = ["GAME.CFG"]`
- **THEN** the folder it runs in is `build`, so a file it writes by name alone stays in `build`

#### Scenario: An output folder without a copy list
- **WHEN** the settings file has `output = "out"` and no `copy` list
- **THEN** the folder it runs in is `out`, and a data file beside the source is not found by name alone

### Requirement: The paths of a program
`qb64rust --print-paths <file.bas>` SHALL print one line of JSON with `exe`, the absolute path that `-x` with no
`-o` would write, and `run_in`, the absolute folder the program runs in, which is the folder of `exe`, after reading
the settings file. With `--target wasm` the line SHALL also have `page`, the absolute path of the `index.html` a
window program's build would write; `exe` is then the script of a `$CONSOLE:ONLY` program. It SHALL take `--target` and `--no-settings` as a build does, build and copy nothing, and exit
with status 0. A settings file that cannot be used SHALL be reported as a build reports it, with status 1.

#### Scenario: Without a settings file
- **WHEN** `<dir>/game.bas` has no settings file and `qb64rust --print-paths <dir>/game.bas` runs on Windows
- **THEN** it prints `{"exe":"<dir>\\game.exe","run_in":"<dir>"}` and nothing is built

#### Scenario: With a copy list
- **WHEN** the settings file beside `<dir>/game.bas` has `copy = ["GAME.CFG"]`
- **THEN** it prints `{"exe":"<dir>\\build\\game.exe","run_in":"<dir>\\build"}`

#### Scenario: With an output folder
- **WHEN** the settings file beside `<dir>/game.bas` has `output = "out"` and no `copy` list
- **THEN** it prints `{"exe":"<dir>\\out\\game.exe","run_in":"<dir>\\out"}`

#### Scenario: For the wasm target
- **WHEN** `qb64rust --print-paths <dir>/game.bas --target wasm` runs on Windows, with no settings file
- **THEN** the line has `"exe":"<dir>\\build\\game.js"` and `"page":"<dir>\\build\\web\\index.html"`, since a
  wasm build keeps the folder `build`
