# Spec Delta

## ADDED Requirements

### Requirement: Programs compared in parallel
The check SHALL take `--jobs <n>` as the test runner does and compare up to `n` programs at once, the new
compiler's C++ of each written in a folder of its own. The old compiler's C++ SHALL come from the per-program
cache when it has it, and SHALL be written one program at a time when it has not, since the old compiler builds
in one folder. The report SHALL be the same, in the same order, whatever `n` is.

#### Scenario: Eight at once
- **WHEN** the check runs with `--jobs 8` over its programs with the old compiler's cache warm
- **THEN** it finishes in about an eighth of the time of `--jobs 1`, and its report is identical

#### Scenario: A cold cache
- **WHEN** a program has no cached C++ of the old compiler
- **THEN** that program's old side is written under the clone's lock while the other programs' new sides go on
