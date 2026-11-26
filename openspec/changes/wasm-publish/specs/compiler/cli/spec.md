# Spec Delta

## MODIFIED Requirements

### Requirement: The output of a wasm build
With `--target wasm` the default output of a `$CONSOLE:ONLY` program SHALL be the source's name with `.js`, in
the output folder (requirement "The output folder"), and the `.wasm` file SHALL have the same name beside it.
The default output of a window program SHALL be the web folder, `web` in the output folder (spec
`compiler/wasm-target`, "A window program runs in a page"), and the build SHALL print the path of its
`index.html`. `-o <file>` SHALL
name the `.js` file and SHALL end in `.js`; another ending is an error with status 1. On failure neither file SHALL be left at the output path.

#### Scenario: A named output
- **WHEN** `qb64rust -x --target wasm p.bas -o out/app.js` builds
- **THEN** `out/app.js` and `out/app.wasm` exist

#### Scenario: A window program
- **WHEN** `qb64rust -x --target wasm game.bas` builds a program without `$CONSOLE:ONLY`
- **THEN** the last line it prints is the path of `build/web/index.html`, and `build/game.js` is not written

#### Scenario: A wrong ending
- **WHEN** `qb64rust -x --target wasm p.bas -o p.exe` runs
- **THEN** the exit status is 1 and the message says the output of a wasm build ends in `.js`

#### Scenario: A failed build
- **WHEN** the C++ build of a wasm program fails and `p.js` and `p.wasm` of an earlier build exist
- **THEN** both are gone after the command, as an executable is after a failed build

### Requirement: The paths of a program
`qb64rust --print-paths <file.bas>` SHALL print one line of JSON with `exe`, the absolute path that `-x` with no
`-o` would write, and `run_in`, the absolute folder the program runs in, after reading the settings file. With
`--target wasm` the line SHALL also have `page`, the absolute path of the `index.html` a window program's
build would write; `exe` is then the script of a `$CONSOLE:ONLY` program. It
SHALL take `--target` and `--no-settings` as a build does, build and copy nothing, and exit with status 0. A
settings file that cannot be used SHALL be reported as a build reports it, with status 1.

#### Scenario: Without a settings file
- **WHEN** `<dir>/game.bas` has no settings file and `qb64rust --print-paths <dir>/game.bas` runs on Windows
- **THEN** it prints `{"exe":"<dir>\\build\\game.exe","run_in":"<dir>"}` and nothing is built

#### Scenario: With a copy list
- **WHEN** the settings file beside `<dir>/game.bas` has `copy = ["GAME.CFG"]`
- **THEN** it prints `{"exe":"<dir>\\build\\game.exe","run_in":"<dir>\\build"}`

#### Scenario: For the wasm target
- **WHEN** `qb64rust --print-paths <dir>/game.bas --target wasm` runs on Windows
- **THEN** the line has `"exe":"<dir>\\build\\game.js"` and `"page":"<dir>\\build\\web\\index.html"`

## ADDED Requirements

### Requirement: Serve subcommand
`qb64rust serve` SHALL serve a web folder as spec `compiler/web-serve` says. The usage text SHALL name it with
its options `--port`, `--open` and `--no-headers`. An option it does not have SHALL be an error with status 1
that prints the usage.

#### Scenario: Usage
- **WHEN** `qb64rust --help` runs
- **THEN** the text has a line for `qb64rust serve`

#### Scenario: An unknown option
- **WHEN** `qb64rust serve build/web --lan` runs
- **THEN** the exit status is 1 and nothing is served
