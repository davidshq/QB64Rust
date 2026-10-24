# Spec Delta

## ADDED Requirements

### Requirement: _RED of a colour beyond the palette in a CONST
`_RED`, `_GREEN` and `_BLUE` in a `CONST` of a colour above 255 and within the mask of a screen mode 0 image
SHALL give 0. The old compiler reads past libqb's palette of 256 entries there, so its value comes from the
memory of its own run (`CLAUDE.md` working rule 12): 0 in most builds, 32, 1 or 179 in others, and a crash of
`qb64pe.exe` for `_RED(65535, 0)` (measured, `verification\v57` task 1.1). A row of `DIVERGENCES.md` names the case.

#### Scenario: A colour above 255 in screen mode 0
- **WHEN** a program has `CONST c = _RED(2222, 0)` and prints `c`
- **THEN** it compiles and prints `accepted: 0` (`verification\v45_x64`)

#### Scenario: The other components
- **WHEN** a program has `CONST g = _GREEN(300, 0)` and `CONST b = _BLUE(65535, 0)`
- **THEN** each is 0

#### Scenario: Screen mode 13
- **WHEN** a program has `CONST k = _RED(256, 13)`
- **THEN** the compile fails with an error, as the old compiler's ("UNEXPECTED INTERNAL COMPILER ERROR!",
  `verification\v57_c05`–`c08`)
