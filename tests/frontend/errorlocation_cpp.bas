' TEST: cpp
' $ERRORLOCATION:ON (m4-builtins-remaining design D6; the old compiler's C++ of verification\v58_e26):
' error_track_line(line,0,NULL) before each statement's code, after a label's or a line number's, for block
' headers, branches and closers too, none for a label or the console switch; in an included file the main file's
' line, the file's line and its name, as evnt has them; in a SUB the same.
$CONSOLE:ONLY
$ERRORLOCATION:ON
DIM a(3) AS LONG
10 PRINT "start"
IF a(1) = 2 THEN PRINT "one" ELSE PRINT "no"
FOR i = 1 TO 2
    PRINT i
NEXT
SELECT CASE i
    CASE 2
        PRINT "two"
    CASE ELSE
        PRINT "else"
END SELECT
'$INCLUDE:'inc/errorlocation.bi'
label1:
PRINT "end"
incsub
SYSTEM

' A DYNAMIC library's load (error 259) and lookup (error 260) record the line of the DECLARE and of the FUNCTION
' first, inside the failure's braces, since they run before any statement (qb64pe.bas 5253-5274, 5874-5895).
DECLARE DYNAMIC LIBRARY "kernel32"
    FUNCTION GetTickCount&
END DECLARE

SUB s (t$)
    PRINT "sub "; t$
END SUB
