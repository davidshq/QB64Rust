## ADDED Requirements

### Requirement: _MD5$
`_MD5$(s$)` SHALL give the MD5 digest of `s$` as 32 upper-case hexadecimal digits, through the old compiler's
runtime call, and a program that names it SHALL link the part of libqb the old compiler links for it.

#### Scenario: A digest
- **WHEN** `PRINT _MD5$("QB64 Phoenix Edition"); " "; _MD5$("")` runs
- **THEN** it prints `E512ECA19E9487D7C2F564E848314238 D41D8CD98F00B204E9800998ECF8427E`
