# Spec Delta

## MODIFIED Requirements

### Requirement: Included files
When the parser meets `$INCLUDE`, the server SHALL look the file up as the compiler does (the including file's
folder, then the include root from the initialization options, never the workspace folder) and SHALL use the open
document's current text when the file is open in the editor, the file on disk otherwise. The client SHALL give as
the include root `qb64rust.includeRoot` when it is set, else the folder of QB64pe when it is found, else the
folder of the `qb64rust` binary, the same root a build with `qb64rust` is given.

#### Scenario: Unsaved include
- **WHEN** `lib.bi` is open with an unsaved syntax error and the main file that includes it is edited
- **THEN** the main program's parse reports the error under `lib.bi`'s uri

#### Scenario: Include edited reparses the includer
- **WHEN** `lib.bi` is edited while `main.bas`, which includes it, is also open
- **THEN** `main.bas`'s program is parsed again and its diagnostics updated

#### Scenario: Include from QB64pe's folder
- **WHEN** `qb64rust.includeRoot` is empty, QB64pe is found in folder `Q`, and a program includes a file that
  is not next to it but exists under `Q`
- **THEN** the server reads the file under `Q`, as a build with `qb64rust` would
