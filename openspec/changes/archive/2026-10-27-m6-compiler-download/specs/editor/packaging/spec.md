# Spec Delta

## MODIFIED Requirements

### Requirement: Release from a tag
Pushing a tag `vscode-v<version>`, where `<version>` equals the version in the extension's `package.json` and the
workspace version in `Cargo.toml`, SHALL build the binary, package the `win32-x64` `.vsix` and the compiler
download of spec `compiler/download` from that one binary, check each against its contents requirement, and
attach both to a GitHub release of that tag. A tag whose version differs from either SHALL fail the release
without publishing anything, as SHALL a failed check of either package.

#### Scenario: Matching tag
- **WHEN** the tag `vscode-v0.2.0` is pushed, and `package.json` and `Cargo.toml` both say `0.2.0`
- **THEN** a release `vscode-v0.2.0` exists with `qb64rust-win32-x64-0.2.0.vsix` and
  `qb64rust-win-x64-0.2.0.zip` attached, and the `qb64rust.exe` in each is the same file

#### Scenario: Mismatched tag
- **WHEN** the tag `vscode-v0.2.1` is pushed and `package.json` says `0.2.0`
- **THEN** the workflow fails before packaging and no release is created

#### Scenario: The compiler's version differs
- **WHEN** the tag `vscode-v0.3.0` is pushed, `package.json` says `0.3.0` and `Cargo.toml` says `0.2.0`
- **THEN** the workflow fails before building, naming both versions, and no release is created

#### Scenario: The download fails its check
- **WHEN** the `.vsix` passes its check and the zip does not
- **THEN** no release is created and neither file is published
