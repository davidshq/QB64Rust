# language/data-processing Specification

## Purpose
The checksum, compression and base64 functions (`_ADLER32`, `_CRC32`, `_DEFLATE$`, `_INFLATE$`, `_BASE64ENCODE$`,
`_BASE64DECODE$`) and the part of libqb a program that names one links.

## Requirements

### Requirement: Checksums, compression and base64
`_ADLER32(s$)`, `_CRC32(s$)`, `_DEFLATE$(s$[, level])`, `_INFLATE$(s$[, size])`, `_BASE64ENCODE$(s$)` and
`_BASE64DECODE$(s$)` SHALL compile to the libqb calls the old compiler writes, with its argument conversions and
result types (`_ADLER32` and `_CRC32` an `_UNSIGNED LONG`, the others strings), and give its values. Their wrong
forms SHALL be the old compiler's errors.

#### Scenario: Round trip
- **WHEN** `PRINT _INFLATE$(_DEFLATE$("abc")) = "abc"; _BASE64DECODE$(_BASE64ENCODE$("abc"))` runs
- **THEN** it prints `-1 abc`

#### Scenario: Checksums
- **WHEN** `PRINT _CRC32("abc"); _ADLER32("abc")` runs
- **THEN** it prints what the old compiler's program prints

#### Scenario: The compressed bytes
- **WHEN** `PRINT LEN(_DEFLATE$("this is a test", 9))` runs
- **THEN** it prints what the old compiler's program prints

#### Scenario: Without the dollar sign
- **WHEN** a program names `_DEFLATE("x")`
- **THEN** it is not compiled: "not supported yet", as for every built-in that needs `$` named without it
  (`_TRIM(...)`; the old compiler's "Invalid array name" waits with arrays used without `DIM`, `SOMEDAY.md`)

### Requirement: The runtime part of the data-processing functions
A program that names one of these functions SHALL be linked with libqb's data-processing part, as the old
compiler's build of it is (`DEP_ZLIB=y`, in the old compiler's order among the make flags); a program that names
none SHALL be linked as before.

#### Scenario: Built with the part
- **WHEN** a program that names `_CRC32` is built
- **THEN** the make line has `DEP_ZLIB=y` and the program runs

#### Scenario: A function in code that is never reached
- **WHEN** `_INFLATE$` stands only in a SUB that is never called
- **THEN** the program is still linked with the part, as with the old compiler

### Requirement: _MD5$
`_MD5$(s$)` SHALL give the MD5 digest of `s$` as 32 upper-case hexadecimal digits, through the old compiler's
runtime call, and a program that names it SHALL link the part of libqb the old compiler links for it.

#### Scenario: A digest
- **WHEN** `PRINT _MD5$("QB64 Phoenix Edition"); " "; _MD5$("")` runs
- **THEN** it prints `E512ECA19E9487D7C2F564E848314238 D41D8CD98F00B204E9800998ECF8427E`
