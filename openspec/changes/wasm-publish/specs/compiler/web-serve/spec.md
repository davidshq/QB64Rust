# Spec Delta

## Purpose

`qb64rust serve` shows a program built for the browser on the user's own machine, served the way a web host
must serve it, so that a game is tried before it is uploaded.

## ADDED Requirements

### Requirement: Serving a web folder
`qb64rust serve <folder>` SHALL serve the files of the folder over HTTP until it is interrupted. `GET` and
`HEAD` of `/` SHALL give the folder's `index.html`, and of a file's path the file. A path outside the folder,
a folder without `index.html`, or a missing file SHALL give status 404, and another method status 405. It
SHALL write nothing into the folder.

#### Scenario: A built game
- **WHEN** `qb64rust serve build/web` runs on the web folder of a built window program and a browser opens the
  address it printed
- **THEN** the program runs in the page as it does in the test launcher

#### Scenario: A path outside the folder
- **WHEN** a request asks for `/../qb64rust.toml`
- **THEN** the answer is status 404 and no file outside the folder is read

#### Scenario: Not a folder
- **WHEN** `qb64rust serve nowhere` runs and `nowhere` does not exist
- **THEN** it exits with status 1 and a message that names the folder

### Requirement: The folder of a program
`qb64rust serve <file.bas>` SHALL serve the web folder a wasm build of that program writes, read from the
settings file as a build reads it. `qb64rust serve` alone SHALL serve `build/web` of the current folder. When
the folder has no `index.html`, it SHALL exit with status 1 and name the build command that writes it.

#### Scenario: By program
- **WHEN** `qb64rust -x --target wasm game.bas` built a page and `qb64rust serve game.bas` runs
- **THEN** it serves `build/web` beside the settings file, or beside the source when there is none

#### Scenario: Not built yet
- **WHEN** `qb64rust serve game.bas` runs before any wasm build
- **THEN** it exits with status 1 and the message names `qb64rust -x --target wasm game.bas`

### Requirement: Headers and content types
Every answer SHALL carry `Cross-Origin-Opener-Policy: same-origin`, `Cross-Origin-Embedder-Policy:
require-corp` and `Cache-Control: no-cache`. A `.wasm` file SHALL be `application/wasm`, a `.html` file
`text/html`, a `.js` file `text/javascript`, and a file of an ending it does not know `application/octet-stream`. With
`--no-headers` the two cross-origin headers SHALL be left out, as on a host that cannot send them.

#### Scenario: The wasm file
- **WHEN** a browser asks for `game.wasm`
- **THEN** the content type is `application/wasm` and both cross-origin headers are present

#### Scenario: A host without the headers
- **WHEN** `qb64rust serve build/web --no-headers` runs and a browser opens the page
- **THEN** the page starts after one reload, through its service worker

### Requirement: The address
The server SHALL listen on `127.0.0.1` only, never on an address other machines reach. The port SHALL be
8064, or the next free port up to 8083 when it is taken. `--port <n>` SHALL name the port, and a taken port
is then an error with status 1. It SHALL print the address as `http://127.0.0.1:<port>/` on a line of its
own before it serves, and with `--open` it SHALL open that address in the user's browser.

#### Scenario: The default port
- **WHEN** `qb64rust serve build/web` runs and port 8064 is free
- **THEN** it prints `http://127.0.0.1:8064/`

#### Scenario: A taken port
- **WHEN** port 8064 is taken and port 8065 is free
- **THEN** it serves on 8065 and prints that address, with a note that saved games belong to the address

#### Scenario: A named port that is taken
- **WHEN** `qb64rust serve build/web --port 9000` runs and port 9000 is taken
- **THEN** it exits with status 1 and the message names the port

#### Scenario: Another machine
- **WHEN** another machine of the network asks for the port
- **THEN** it gets no answer, since nothing listens on that address
