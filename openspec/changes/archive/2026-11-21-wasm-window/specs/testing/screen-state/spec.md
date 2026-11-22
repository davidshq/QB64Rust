## ADDED Requirements

### Requirement: Screen programs on the wasm target
With `--platform wasm`, a screen program of the corpus SHALL be built for a browser page and run in headless
Chrome by `tools\wasm\browser.mjs`, and judged by the same `.screen` file, keys file and output as on the desktop
(as Linux, by its Linux sidecars). The launcher SHALL open no port. It SHALL write the program's folder and the
files the test's variables name into the page's file system, and write back the state file when the program ends.

#### Scenario: A screen program in a browser
- **WHEN** `run_legacy_tests.py --suite corpus --platform wasm` runs `screen/c02_screen12_shapes`
- **THEN** the program runs in headless Chrome and passes only if the state written equals
  `c02_screen12_shapes.screen`

#### Scenario: Keys in a browser
- **WHEN** it runs `screen/c05_inkey_keys`, whose `.keys` file types keys
- **THEN** the keys reach the program's `INKEY$` and the state equals the recording

#### Scenario: No port
- **WHEN** the launcher runs a page
- **THEN** Chrome is driven over a pipe, and no process of the run listens on a port
