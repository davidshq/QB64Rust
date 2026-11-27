# Spec Delta

## ADDED Requirements

### Requirement: Run in browser
The command "QB64: Run in Browser" (`qb64rust.runInBrowser`) SHALL build the active document with `qb64rust`
for the wasm target and, only if the build succeeds and wrote a page, serve the page's folder with `qb64rust
serve` in a terminal named "QB64 Web" and open the address it prints in the user's browser. The build's
errors SHALL be shown as a build's are. The command SHALL be unavailable in untrusted workspaces, as builds
are.

#### Scenario: A window program
- **WHEN** the user runs the command on a correct window program with the Emscripten SDK found
- **THEN** the browser opens the page and the program runs in it

#### Scenario: A second run
- **WHEN** the user changes the program and runs the command again while the first server still runs
- **THEN** the game is built again, the old server is stopped, and the browser opens the same address, so the
  saved games of the first run are found

#### Scenario: A console program
- **WHEN** the user runs the command on a `$CONSOLE:ONLY` program
- **THEN** nothing is served, and a message says that such a program has no page and runs under Node

#### Scenario: A build error
- **WHEN** the program uses `SHELL`, which the wasm target does not have
- **THEN** the error is in the Problems view at its line, and nothing is served

#### Scenario: Built with QB64pe
- **WHEN** `qb64rust.compiler` is `qb64pe`
- **THEN** the command says that the browser build needs `qb64rust`, and nothing runs
