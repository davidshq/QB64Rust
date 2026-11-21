# Spec Delta

## MODIFIED Requirements

### Requirement: Supported platforms
`qb64rust` SHALL build, check, format, serve the editor and debug on Windows x64 and on Linux x64 (glibc). A build
SHALL target the platform `qb64rust` runs on, unless `--target wasm` names the WebAssembly target (spec
`compiler/wasm-target`). There is no other cross-compilation. On any other platform, building SHALL
fail with a message naming the platforms supported. Checking (`-z`, `--dump`, `lsp`, `-y`) SHALL still work there.

#### Scenario: Build on Linux
- **WHEN** `qb64rust -x hello.bas` runs on Linux x64 with the system toolchain present
- **THEN** an ELF executable `hello` is written beside the source, and it runs

#### Scenario: Checking needs no supported platform
- **WHEN** `qb64rust lsp` runs on a platform that is not supported
- **THEN** the language server works as on Windows

#### Scenario: A build for WebAssembly on Windows
- **WHEN** `qb64rust -x --target wasm hello.bas` runs on Windows x64 with the Emscripten SDK found
- **THEN** `hello.js` and `hello.wasm` are written, and no `hello.exe`
