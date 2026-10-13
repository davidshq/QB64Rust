' Slice (m2-console-screenhide, group 6): the precompiler flags in $IF, with the values the old compiler gives
' them from the whole program. This program needs three rounds: its only OPTION _EXPLICIT stands inside the
' branch its own flag switches off (met in the first round, in force by the flag from then on), and its only
' $CONSOLE inside $IF _EXPLICIT_ (met in the second round). Every line is then read with _CONSOLE_ 1 and
' _EXPLICIT_ 1, also the lines above the ones that set them. The two programs beside it (suffixes b and c) hold
' the settings one program cannot hold together.
$SCREENHIDE
_DEST _CONSOLE
$IF _CONSOLE_ = 1 THEN
    PRINT "above the line of $CONSOLE: 1"
$ELSEIF _CONSOLE_ = 2 THEN
    PRINT "above the line of $CONSOLE: 2"
$ELSE
    PRINT "above the line of $CONSOLE: 0"
$END IF
$IF _EXPLICIT_ THEN
    PRINT "above the option: explicit 1"
    $CONSOLE
$ELSE
    PRINT "above the option: explicit 0"
$END IF
$IF _EXPLICIT_ = 0 THEN
    OPTION _EXPLICIT
    PRINT "the branch of the option is compiled"
$END IF
$IF _EXPLICITARRAY_ THEN
    PRINT "explicitarray 1"
$ELSE
    PRINT "explicitarray 0"
$END IF
$IF _ASSERTS_ OR _DEBUG_ OR _SOCKETS_ THEN
    PRINT "another flag is set"
$ELSE
    PRINT "asserts, debug, sockets 0"
$END IF
$IF _CONSOLE_ = 1 AND _EXPLICIT_ = 1 THEN
    PRINT "console 1 and explicit 1"
$END IF
$IF _CONSOLE_ <> 0 AND WINDOWS THEN
    PRINT "a flag beside a predefined name"
$END IF
$IF _CONSOLE_ = 0 THEN
    PRINT "not printed"
    $IF _EXPLICIT_ THEN
        PRINT "not printed either"
    $END IF
$ELSEIF _CONSOLE_ >= 1 THEN
    PRINT "console >= 1"
$END IF
DIM declared AS LONG
declared = _CONSOLE
PRINT "the console's handle:"; declared
show
SYSTEM

SUB show
    $IF _CONSOLE_ THEN
        PRINT "in a SUB: console"
    $ELSE
        PRINT "in a SUB: no console"
    $END IF
END SUB
