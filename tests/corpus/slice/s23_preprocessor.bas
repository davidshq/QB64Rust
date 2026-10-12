$CONSOLE:ONLY
' The preprocessor (m2-parser-breadth task 7.1, design D8): predefined names on Windows 64-bit, $LET, $IF with
' $ELSEIF/$ELSE IF/$ELSE and both $END IF spellings, nested $IFs (a true one inside a skipped branch stays
' skipped), comparisons (=, <>, <, >=, VERSION), AND/OR/XOR, DEFINED/UNDEFINED, $LET of a predefined name not
' overriding it, a $IF inside a FOR body and a SUB, a whole SUB inside an active $IF, and skipped branches holding
' code that would not compile (an unclosed FOR, a SUB, garbage, an unknown metacommand).
$IF WIN THEN
    PRINT "win"
$ELSE
    PRINT "not win"
$END IF
$IF LINUX THEN
    PRINT "linux"
$ELSEIF MAC THEN
    PRINT "mac"
$ELSE IF 64BIT THEN
    PRINT "64bit"
$ELSE
    PRINT "else"
$ENDIF
$IF 32BIT OR _ARM_ THEN
    PRINT "32bit or arm"
    FOR i = 1 TO
$ELSE
    PRINT "neither 32bit nor arm"
$END IF
$LET MODE = 2
$IF MODE = 1 THEN
    PRINT "mode 1"
$ELSEIF MODE = 2 THEN
    PRINT "mode 2"
$ELSEIF MODE >= 2 THEN
    PRINT "mode >= 2, not taken after a taken branch"
$END IF
$LET MODE = 3
$IF MODE > 2 AND WIN THEN
    PRINT "mode > 2 and win"
$END IF
$IF MODE <> 3 XOR WIN THEN
    PRINT "xor"
$END IF
$IF VERSION >= 4.0 THEN
    PRINT "version >= 4.0"
$END IF
$IF VERSION < 3.9.1 THEN
    PRINT "version < 3.9.1"
$END IF
$IF NOTDEFINED = UNDEFINED THEN
    PRINT "undefined"
$END IF
$IF MODE = DEFINED THEN
    PRINT "defined"
$END IF
$LET WIN = 0
$IF WIN THEN
    PRINT "win still true"
$END IF
$IF LINUX THEN
    $IF WIN THEN
        PRINT "nested, skipped"
    $ELSE
        this is not BASIC (
    $END IF
    $FOO:BAR
    SUB never
    PRINT "never"
$ELSE
    $IF 64BIT THEN
        PRINT "nested, taken"
    $END IF
$END IF
FOR i = 1 TO 2
    $IF WIN THEN
        PRINT "loop"; i
    $END IF
NEXT
s
$IF WIN THEN
    t
$END IF
SYSTEM

SUB s
    $IF MAC THEN
        PRINT "mac sub"
    $ELSE
        PRINT "sub"
    $END IF
END SUB

$IF WIN THEN
SUB t
    PRINT "t"
END SUB
$END IF
