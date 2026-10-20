## MODIFIED Requirements

### Requirement: Errors of $EXEICON
The compiler SHALL report an error that is not "not supported yet" for a `$EXEICON` line without a name between
single quotes, for a second `$EXEICON` line of a program (whatever it names), for a name whose file is not
there, and for a file that is there but is no icon file, each at the line, as the old compiler does (for the
last, its C++ build fails). A relative name that does not start with `./` or `.\` SHALL be
reported "not supported yet". The line rules of every metacommand SHALL hold for it.

#### Scenario: No name
- **WHEN** a program has `$EXEICON`, `$EXEICON:''`, `$EXEICON:"./icon.ico"` or `$EXEICON:'./icon.ico`
- **THEN** the compiler reports that it expected `$EXEICON:'filename'`

#### Scenario: Twice
- **WHEN** a program has `$EXEICON:'./icon.ico'` on line 4 and a `$EXEICON` line further down
- **THEN** the compiler reports at the second line that `$EXEICON` is already defined in line 4

#### Scenario: A missing file
- **WHEN** a program has `$EXEICON:'./missing.ico'` and no such file lies beside it
- **THEN** the compiler reports that the file is not found, naming it as it is written in the program

#### Scenario: A file that is no icon
- **WHEN** a program has `$EXEICON:'./v29_inc/not_an_icon.ico'` and that file holds text
- **THEN** the compiler reports at the line, before any C++ is built, that the file is not an icon file, naming
  it as it is written in the program (`verification\v29_e16_not_an_icon`); an icon file of one or of five images
  gives no diagnostic

#### Scenario: A relative name without ./
- **WHEN** a program has `$EXEICON:'icons/icon.ico'` or `$EXEICON:'/icons/icon.ico'`
- **THEN** the compiler reports "not supported yet" at the line (the old compiler looks in its own folder, or on
  its own drive)

#### Scenario: After a colon, and a tab before it
- **WHEN** a program has `l = 1: $EXEICON:'./icon.ico'` or a tab before `$EXEICON`
- **THEN** the compiler reports an error, as for any metacommand
