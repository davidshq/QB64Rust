# Spec Delta

## ADDED Requirements

### Requirement: A built-in statement written with a suffix
A built-in statement written with a type suffix it does not take (`KILL$`, `_ASSERT$`) SHALL be a compile error
that is not "not supported yet", as the old compiler's "Syntax error" is. A statement and suffix that
`verification\v55` does not record SHALL stay "not supported yet".

#### Scenario: KILL$ and _ASSERT$
- **WHEN** `KILL$ "a"` or `_ASSERT$ 1` is compiled
- **THEN** each is a compile error that is not "not supported yet" (`verification\v22_x20_kill_suffix`,
  `v33_x50_assert_dollar`)
