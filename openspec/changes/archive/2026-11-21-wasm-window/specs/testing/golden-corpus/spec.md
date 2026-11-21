## MODIFIED Requirements

### Requirement: The wasm known list
Each suite MAY have `known.wasm.list`, in the format of `known.list`, read beside the shared `known.list` and
the suite's `known.linux.list`. Every line's reason SHALL start with one of: `target`, `float`, `clib`, `width`,
`node`, `varies`, `open`. A program listed as `target` SHALL be skipped, not run.

#### Scenario: What the reasons mean
- **WHEN** a program is put on the list
- **THEN** its reason is `target` if the compiler rejects it for this target, `float` for a float result or
  conversion that depends on the machine (a `_FLOAT` value, a float converted to an integer outside its range),
  `clib` for text or a value from the C library, `width` for what this target is, a 32-bit one that is not Linux
  (`$IF LINUX`, `$IF 32BIT`, `_OS$`, an index or a size past 32 bits, a size or a value that comes from a
  pointer's 4 bytes), `node` for what Node presents differently (the console, the environment, the file system),
  `varies` as in the other lists, and `open` if it is not explained yet

#### Scenario: A window program
- **WHEN** a program without `$CONSOLE:ONLY` is not on the list and the suite runs with `--platform wasm`
- **THEN** it is built with a page and run in headless Chrome, and judged by its recordings as a console program
  is

#### Scenario: A reason outside the set
- **WHEN** a line of `known.wasm.list` gives the reason `later`, `offset` or `window`
- **THEN** the runner's check of the lists fails and names the line

#### Scenario: A listed program that passes
- **WHEN** a program listed as `float` gives its recorded output
- **THEN** the run reports it as a new pass, as for the other known lists
