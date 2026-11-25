# Spec Delta

## MODIFIED Requirements

### Requirement: Source is bytes
The compiler SHALL read source files as bytes and SHALL NOT require them to be valid UTF-8, except a file whose
encoding is UTF-8 (spec `compiler/source-encoding`). Every position it reports or stores SHALL be a file and a
byte offset; columns in diagnostics SHALL be 1-based byte columns.

#### Scenario: CP437 bytes in a string literal
- **WHEN** a program contains `PRINT "` followed by the bytes 0xC9 0xCD 0xBB and `"`
- **THEN** it compiles, and the executable writes exactly those three bytes followed by the line end

#### Scenario: UTF-8 bytes in a file that declares nothing
- **WHEN** a program with no `'$ENCODING` line and no settings file contains `PRINT "` followed by the bytes
  0xE2 0x94 0x80 and `"`
- **THEN** it compiles, and the executable writes exactly those three bytes followed by the line end
