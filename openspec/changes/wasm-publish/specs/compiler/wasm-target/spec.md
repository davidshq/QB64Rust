# Spec Delta

## MODIFIED Requirements

### Requirement: A window program runs in a page
A program without `$CONSOLE:ONLY` built with `--target wasm` SHALL be built for a browser. Without `-o` the
build SHALL write the page as `index.html` into the web folder, `web` in the output folder, with `<name>.js`
and `<name>.wasm` beside it. With `-o <file>.js` it SHALL write `<file>.html`, `<file>.js` and `<file>.wasm`.
Opening the page from a host that sends the two headers of threads
(`Cross-Origin-Opener-Policy`, `Cross-Origin-Embedder-Policy`) SHALL run the program in the page's canvas, with
the keyboard and the mouse, and show the screen its desktop build shows. A `$CONSOLE:ONLY` build SHALL remove a
`<name>.html` left beside its `.js` file.

#### Scenario: A drawing
- **WHEN** `qb64rust -x --target wasm game.bas` builds `SCREEN 12: LINE (10, 10)-(100, 50), 4, BF: SYSTEM`
- **THEN** `index.html`, `game.js` and `game.wasm` are in `build/web`, and the page, run in headless
  Chrome, ends with exit status 0 and a screen state equal to the desktop build's

#### Scenario: A named output
- **WHEN** the same program is built with `-o out/game.js`
- **THEN** `out/game.html`, `out/game.js` and `out/game.wasm` exist, and no `web` folder is written

#### Scenario: Keys
- **WHEN** a page's program waits in `INKEY$` and a key is typed over its canvas
- **THEN** `INKEY$` returns the key as on the desktop

#### Scenario: A console program afterwards
- **WHEN** `game.bas` is changed to `$CONSOLE:ONLY` and built again with `--target wasm`
- **THEN** `game.html` no longer exists

### Requirement: A page's files
A wasm build of a window program without `-o` SHALL write the files that `[build]` `copy` names into
`<name>.data` in the web folder, which SHALL put them in the program's folder, at their paths in the output
folder, before it starts. The page SHALL keep what the
program writes there in the browser's storage across visits; a shipped file the program changed SHALL stay
changed until the page ships another version of it.

#### Scenario: A shipped file
- **WHEN** `qb64rust.toml` has `[build]` `copy = ["data"]` and the program reads `data/hello.txt`
- **THEN** the page's program reads the file, though the web folder holds no `data` folder

#### Scenario: A file kept across visits
- **WHEN** the program appends a line to `save.txt` and the page is opened three times in one browser
- **THEN** the third run counts three lines

#### Scenario: A new version of a shipped file
- **WHEN** the program changed its shipped `data/score.txt`, and the game is built again with another
  `data/score.txt`
- **THEN** the next visit reads the new version

#### Scenario: A build with -o
- **WHEN** the same program is built with `-o out/game.js`
- **THEN** no `out/game.data` is written, as nothing of `[build]` `copy` is copied

## ADDED Requirements

### Requirement: The web folder is uploaded whole
The web folder SHALL hold what a host needs and nothing else of the build: `index.html`, `<name>.js`,
`<name>.wasm`, `<name>.data` when the project has a `copy` list, `_headers` and `qb64rust-sw.js`. The build
SHALL replace each of these and remove a `<name>.data` that it no longer writes. It SHALL leave any other file
of the folder alone. A `$CONSOLE:ONLY` build SHALL write no web folder.

#### Scenario: Beside a desktop build
- **WHEN** a project with a `copy` list is built for the desktop and then for wasm
- **THEN** `build/web` holds the six files, and no executable, saved game or copied data folder

#### Scenario: The copy list removed
- **WHEN** the `copy` list is removed from the settings file and the game is built again
- **THEN** `build/web/<name>.data` is gone

#### Scenario: A file of the user's
- **WHEN** the user put `favicon.ico` into `build/web` and the game is built again
- **THEN** `favicon.ico` is still there

### Requirement: The headers file
The web folder's `_headers` SHALL ask a host that reads such a file for `Cross-Origin-Opener-Policy:
same-origin` and `Cross-Origin-Embedder-Policy: require-corp` on every path.

#### Scenario: The file's text
- **WHEN** a window program is built for wasm without `-o`
- **THEN** `_headers` is the line `/*` followed by the two headers, each indented, one per line

### Requirement: A page on a host without the headers
A page that is not cross-origin isolated SHALL register the web folder's `qb64rust-sw.js` as a service worker
and reload once. The worker SHALL add the two headers to what the page loads, so that the program then starts.
A page that is already isolated SHALL register nothing. A page built with `-o` SHALL have no such step.

#### Scenario: A host that sends no headers
- **WHEN** the web folder is served without the two headers, over HTTPS or from `127.0.0.1`
- **THEN** the page reloads once and the program runs

#### Scenario: A host that sends them
- **WHEN** the web folder is served with the two headers
- **THEN** the program runs without a reload and no service worker is registered

#### Scenario: A second visit
- **WHEN** the page of a host without the headers is opened again in the same browser
- **THEN** the program runs without a reload

### Requirement: A page that cannot start says why
When the page is not cross-origin isolated and a service worker cannot make it so, the page SHALL show a
message in place of "Loading...". The message SHALL name the two headers and say that the host must send them.
The page SHALL not reload more than once for this.

#### Scenario: Opened from the disk
- **WHEN** `index.html` is opened as a file
- **THEN** the page says that it must be opened through a web server, and names `qb64rust serve`

#### Scenario: A worker that does not help
- **WHEN** the page is still not isolated after its one reload, as inside a frame of another site that is not
  isolated
- **THEN** the page shows the message and does not reload again
