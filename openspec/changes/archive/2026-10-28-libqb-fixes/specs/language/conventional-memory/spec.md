# Spec Delta

## ADDED Requirements

### Requirement: BLOAD and BSAVE past the block
A `BLOAD` or `BSAVE` whose start plus length runs past the end of libqb's block of conventional memory SHALL raise
error 5 before reading or writing anything. Inside the block they SHALL work as with the old compiler, wherever
the array their address came from ends (`study\39` row 5; a `QB64PE-DIVERGENCES.md` row).

#### Scenario: A BLOAD at the top of the block
- **WHEN** a program with a handler that prints `ERR` and resumes next runs `DEF SEG = &HFFFF` and `BLOAD f$, &HFFF0`
  for a file saved with a length of 1000
- **THEN** the handler prints 5, and nothing past the block is written

#### Scenario: A file longer than 64 KB with size 0
- **WHEN** a `BLOAD` reads a file whose header gives size 0 and whose data is 200,000 bytes long, to an address
  less than 200,000 bytes from the end of the block
- **THEN** it raises error 5

#### Scenario: The last bytes of the block
- **WHEN** a program saves the block's last 19 bytes with `DEF SEG = &HFFFF` and `BSAVE f$, &HFFF0, 19`, and loads
  them back with `BLOAD f$, &HFFF0`
- **THEN** neither raises an error, as with the old compiler

#### Scenario: A BSAVE at the top of the block
- **WHEN** a program runs `DEF SEG = &HFFFF` and `BSAVE f$, &HFFF0, 1000` under a handler
- **THEN** it raises error 5 and `f$` is not created
