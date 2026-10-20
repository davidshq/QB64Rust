## ADDED Requirements

### Requirement: Errors the C++ build finds
When the C++ build fails and every error of the C++ compiler and the linker names the C function of a declaration
in a block that is not `DYNAMIC`, each error SHALL be reported as an error of the program, not marked "not
supported yet": a compile error at the BASIC line the C++ compiler names, with its message and its notes; an
undefined symbol at the declaration. The summary line SHALL follow and the exit status SHALL be 1. Any other
failure SHALL be reported as the toolchain's output.

#### Scenario: A function the header does not declare
- **WHEN** a block names `"v37_lib/v37h"`, declares `FUNCTION h_none& (BYVAL v AS LONG)`, which the header does
  not declare, and line 6 prints `h_none&(1)`
- **THEN** compiling it fails with an error at line 6 naming `h_none`, then `1 error`, and no executable
  (`verification\v37_x30`; the old compiler's C++ fails)

#### Scenario: The C name in another letter case
- **WHEN** a block names `"v37_lib/v37h"` and declares `FUNCTION H_L& (BYVAL v AS LONG)` where the header has
  `h_l`
- **THEN** compiling it fails with an error at the line that calls it, naming `H_L` (`verification\v37_x31`)

#### Scenario: A function of the library that the header does not declare
- **WHEN** a block names `"v37_lib/v37h", "./v37c"` and declares `c_half#`, which `v37c.a` holds and the header
  does not declare
- **THEN** compiling it fails with an error at the line that calls it, naming `c_half` (`verification\v37_x23`)

#### Scenario: An argument C++ cannot convert
- **WHEN** a header function takes a `const void*`, declared `FUNCTION h_isnull& (BYVAL p AS _OFFSET)`, and the
  program prints `h_isnull&(5)`
- **THEN** compiling it fails with an error at that line naming `h_isnull` that says no conversion from `long
  long` to `const void *` is known (`verification\v37_x17`)

#### Scenario: A function defined nowhere
- **WHEN** a `DECLARE CUSTOMTYPE LIBRARY` block with no name declares `FUNCTION f& ()`, which neither libqb nor the
  C runtime defines
- **THEN** compiling it fails with an error at the declaration naming `f` as not defined (`verification\v37_x04`)

#### Scenario: Another C++ failure
- **WHEN** the C++ build fails with an error that names no library function
- **THEN** the toolchain's output is printed as before, with no summary line, and the exit status is 1
