' TEST: check-ok
$CONSOLE:ONLY
' Precompiler flags (m2-console-screenhide task 6.2, design D7; verification/v26_x54_flag_guarded_explicit): the
' only OPTION _EXPLICIT stands inside the branch its own flag switches off, and _EXPLICIT_ is 1: the branch
' below that would be an error is not compiled.
$IF _EXPLICIT_ = 0 THEN
    OPTION _EXPLICIT
$END IF
$IF _EXPLICIT_ THEN
    DIM ok AS LONG
$ELSE
    DIM ok AS STRING
    ok = 5 +
$END IF
ok = 1
PRINT ok
SYSTEM
