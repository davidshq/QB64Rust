# Spec Delta

## ADDED Requirements

### Requirement: The runner for the wasm target
`run_legacy_tests.py --platform wasm` SHALL build each program with `--target wasm` and run it with the Node
of the Emscripten SDK, on a Windows host and in the suites VM, for the corpus and verification suites.
Standard input and `QB64PE_NOPROMPT=y` SHALL be as on Linux. `--record` with this platform SHALL be refused:
there is no QB64pe build to record from.

#### Scenario: One program by name
- **WHEN** a `$CONSOLE:ONLY` verification program is run with `--platform wasm` on Windows
- **THEN** it is built to `.js` and `.wasm` in its work folder, run under Node, and judged

#### Scenario: Several at a time from a list
- **WHEN** the suite runs with `--platform wasm --jobs 4 --list <file>`
- **THEN** the programs of the list are built and run four at a time, as for the other platforms

#### Scenario: Recording
- **WHEN** `--record --platform wasm` is given
- **THEN** the runner exits with a usage error that says the wasm target has no recordings of its own

### Requirement: What a wasm run is judged by
With `--platform wasm` a program SHALL be judged against its Linux file (`<name>.linux.output`,
`.linux.err`, `.linux.out.txt`) when there is one, else against the shared recording, with the rules a Linux
run is judged by: Linux's words for libqb's own messages, and the exit status by its low 8 bits. No file
named for the wasm platform SHALL be read or written.

#### Scenario: A program with a Linux recording
- **WHEN** a console program whose `.linux.output` holds the Linux console's comma spacing runs with
  `--platform wasm`
- **THEN** it is judged against `.linux.output`

### Requirement: The wasm known list
Each suite MAY have `known.wasm.list`, in the format of `known.list`, read beside the shared `known.list` and
the suite's `known.linux.list`. Every line's reason SHALL start with one of: `target`, `window`, `offset`,
`float`, `clib`, `width`, `node`, `varies`, `open`. A program listed as `target`, `window` or `offset` SHALL be
skipped, not run.

#### Scenario: What the reasons mean
- **WHEN** a program is put on the list
- **THEN** its reason is `target` if the compiler rejects it for this target, `window` if it is not
  `$CONSOLE:ONLY`, `offset` if it is rejected as not available yet for a pointer's size, `float` for a float
  result or conversion that depends on the machine (a `_FLOAT` value, a float converted to an integer outside
  its range), `clib` for text or a value from the C library, `width` for what this target is, a 32-bit
  one that is not Linux (`$IF LINUX`, `$IF 32BIT`, `_OS$`, an index or a size past 32 bits), `node` for what Node presents differently (the
  console, the environment, the file system), `varies` as in the other lists, and `open` if it is not
  explained yet

#### Scenario: A window program
- **WHEN** a program without `$CONSOLE:ONLY` is on `known.wasm.list` as `window` and the suite runs with
  `--platform wasm`
- **THEN** it is counted as skipped with the reason, and no build is started for it

#### Scenario: A reason outside the set
- **WHEN** a line of `known.wasm.list` gives the reason `later`
- **THEN** the runner's check of the lists fails and names the line

#### Scenario: A listed program that passes
- **WHEN** a program listed as `float` gives its recorded output
- **THEN** the run reports it as a new pass, as for the other known lists

### Requirement: The wasm slice list
`tests\wasm\slice.list` SHALL name at most 25 programs of the corpus and verification suites that together
cover: printing numbers of each type, strings, a file read and written, standard input, `ON ERROR`, a run-time
error's exit status, `TIMER` or `_LIMIT`, and `ON TIMER`. Every program on it SHALL pass with
`--platform wasm`.

#### Scenario: The list on the host
- **WHEN** the list is run with `--platform wasm` on a Windows host with the SDK
- **THEN** every program passes, and no window opens and no network port is opened
