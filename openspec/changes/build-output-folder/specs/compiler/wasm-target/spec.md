## MODIFIED Requirements

### Requirement: A wasm build
A build with `--target wasm` SHALL compile the program's C++ and libqb with the Emscripten SDK for `wasm32` and
write two files: `<name>.js` and `<name>.wasm`. `node <name>.js` SHALL run the program. Its standard input,
standard output, exit status, command line and files SHALL be those of the Node process, and its current
folder the folder of `<name>.js`. With no `-o` the two files SHALL be in the output folder (spec `compiler/cli`,
"The output folder"), so a data file the program opens by name alone is found there: in `[build]` `copy`, or
put there by hand.

#### Scenario: Hello
- **WHEN** `qb64rust -x --target wasm hello.bas` builds a `$CONSOLE:ONLY` program that prints `Hello`, with no
  settings file
- **THEN** `build/hello.js` and `build/hello.wasm` are in the source's folder, and `node build/hello.js` prints
  `Hello` and exits with status 0

#### Scenario: A file beside the program
- **WHEN** a wasm program opens `data.txt` for input and the file is in the folder of its `.js` file
- **THEN** it reads the file's lines, whatever folder `node` was started from

#### Scenario: A copied data file
- **WHEN** the settings file beside `hello.bas` has `[build]` and `copy = ["data.txt"]`, and the program opens
  `data.txt`
- **THEN** `qb64rust -x --target wasm hello.bas` copies it to `build/data.txt`, and the program reads it there

#### Scenario: The exit status
- **WHEN** a wasm program ends with `SYSTEM 3`
- **THEN** the Node process exits with status 3
