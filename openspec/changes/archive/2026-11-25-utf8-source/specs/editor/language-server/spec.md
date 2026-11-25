# Spec Delta

## ADDED Requirements

### Requirement: A UTF-8 file open in another encoding
When a document's file is UTF-8 (spec `compiler/source-encoding`, "A file's encoding") and the client names
another encoding for the document, the server SHALL publish one warning at the document's first line. The
warning SHALL say how the file was declared, name the encoding the document is open in, and say to reopen it as
UTF-8. The warning SHALL carry no code. The server SHALL still parse the document's bytes as the client's
encoding gives them. The warning SHALL go when the client names UTF-8 for the document.

#### Scenario: A project's file open as code page 437
- **WHEN** `qb64rust.toml` holds `encoding = "utf-8"` under `[project]`, and `main.bas` is open with the
  client's encoding `cp437`
- **THEN** the document has a warning at line 1 that names `qb64rust.toml` and `cp437`

#### Scenario: Reopened as UTF-8
- **WHEN** the user reopens that document with the encoding UTF-8
- **THEN** the warning is gone

#### Scenario: A file that declares nothing
- **WHEN** a document with no `'$ENCODING` line and no settings file is open as `cp437`
- **THEN** no such warning is published
