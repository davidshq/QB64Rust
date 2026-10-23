# Spec Delta

## MODIFIED Requirements

### Requirement: Compiler root for included files
The compiler root, against which an included file is looked up when it is not in the including file's folder,
SHALL be the folder holding the `qb64rust` executable, unless `--include-root <dir>` names another; a relative
`<dir>` is resolved once, against the working directory at start. Included files SHALL NOT be looked up relative
to the working directory itself (the old compiler changes to its own folder at start). The same root SHALL serve
the relative names of `$EXEICON` and `$EMBED` that do not start with `./` or `.\`.

#### Scenario: Default root
- **WHEN** `qb64rust` runs without `--include-root`
- **THEN** included files not found next to the including file are looked up relative to the executable's folder

#### Scenario: An icon under the root
- **WHEN** `qb64rust --include-root r prog.bas` compiles `$EXEICON:'icons/a.ico'` and `r/icons/a.ico` is an icon
  file
- **THEN** there is no diagnostic and the executable carries that icon, whatever the working directory is
