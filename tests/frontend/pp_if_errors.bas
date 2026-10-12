' TEST: check-fail
$CONSOLE:ONLY
' Preprocessor errors (m2-parser-breadth design D8, task 7.1), each one rejected by the old compiler
' (verification\v16_m6_*, study\00 §5); the messages are new. One error per mistake.
' A block header inside an active $IF, closed outside it:
$IF WIN THEN
IF x THEN
$END IF
END IF
' An IF opened before a $IF and closed inside it:
IF x THEN
$IF WIN THEN
END IF
$END IF
' A FOR closed inside a $IF:
FOR i = 1 TO 2
$IF WIN THEN
NEXT
$END IF
' Conditions and $LET:
$IF NOT LINUX THEN
$END IF
$IF WIN == -1 THEN
$END IF
$IF WIN
$END IF
$LET E
$LET 1A = 2
' $ELSE twice, $ELSEIF after $ELSE, closers without $IF:
$IF WIN THEN
$ELSE
$ELSE
$END IF
$IF WIN THEN
$ELSE
$ELSEIF LINUX THEN
$END IF
$ELSE
$END IF
' After a colon:
x = 1: $LET A = 1
' An active $ERROR (an inactive one is fine):
$IF LINUX THEN
$ERROR not here
$END IF
$ERROR stop here
' A precompiler flag the old compiler sets from the whole program: not supported yet (never evaluated with a guess):
$IF _CONSOLE_ = 2 THEN
$ELSEIF _DEBUG_ THEN
$END IF
' Nested preprocessor lines in an inactive branch are still checked (measured 2026-10-07 with qb64pe -z: "$IF
' without THEN", "Duplicate operator (=)", "$IF block already has $ELSE statement in it"); $LET and $ERROR there
' are not:
$IF LINUX THEN
$IF FOO
$END IF
$IF A == 1 THEN
$END IF
$IF A THEN
$ELSE
$ELSE
$END IF
$LET 1A = 2
$ERROR not checked
$END IF
' Not closed at the end of the file:
$IF WIN THEN
