' TEST: warnings
' `unreachable` (spec `compiler/lints`, "Unreachable code"): one warning per dead stretch, after `GOTO` (line 9),
' `EXIT DO` (line 15), `RETURN` (line 22), `EXIT SUB` (line 41) and `END` (line 46). Quiet: code after a
' single-line `IF … THEN GOTO`, after `ERROR 5`, after `ON n GOTO`, the handler after `END`, `DATA` after it, and
' the code after a `$IF WIN` branch that leaves (`Platform`), and after `RUN file$` (`RunFile`).
$CONSOLE:ONLY
ON ERROR GOTO handler
n = 1
GOTO done
PRINT "dead 1"
PRINT "dead 2"
done:
DO
    PRINT "once"
    EXIT DO
    PRINT "dead 3"
    PRINT "dead 4"
LOOP
GOSUB work
GOTO more
work:
RETURN
PRINT "dead 5"
PRINT "dead 6"
more:
IF n = 2 THEN GOTO 10
PRINT "after the IF"
10 ERROR 5
PRINT "after ERROR"
ON n GOTO last
PRINT "after ON GOTO"
last:
Leave
Finish
Platform: RunFile
END
handler:
RESUME NEXT
DATA 1, 2
SUB Leave
    EXIT SUB
    PRINT "dead 7"
    PRINT "dead 8"
END SUB
SUB Finish
    END
    PRINT "dead 9"
    PRINT "dead 10"
END SUB
SUB Platform
    $IF WIN THEN
        PRINT "windows"
        EXIT SUB
    $END IF
    PRINT "other platforms"
END SUB
SUB RunFile
    RUN "next.bas"
    PRINT "could not start"
END SUB
