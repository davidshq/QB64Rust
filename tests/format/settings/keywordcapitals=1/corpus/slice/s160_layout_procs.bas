$CONSOLE:ONLY
' Slice (m5b-format-settings, task 1.1): SUBs and FUNCTIONs laid out by hand, for the format oracle under the
' other layout settings (tests\format\settings.list): bodies indented wrong on purpose, blocks inside them, a
' comment after the SUB and FUNCTION lines and their ends, and odd spacing.
DECLARE SUB show (t AS STRING)   ' a DECLARE
DIM SHARED total AS LONG
show "start"
PRINT twice(4); twice(5)
CALL show("call")
tally 3
PRINT total
PRINT fact&(5)
END

SUB show (t AS STRING) ' a SUB
    PRINT "show: "; t
END SUB ' its end

FUNCTION twice% (x AS INTEGER) ' a FUNCTION
    twice% = x * 2
END FUNCTION ' its end

SUB tally (n)
    STATIC calls
    calls = calls + 1
    FOR i = 1 TO n
        total = total + i
        IF total > 4 THEN
            EXIT SUB ' leaving early
        END IF
    NEXT
END SUB

FUNCTION fact& (n AS LONG)
    SELECT CASE n
        CASE IS <= 1
            fact& = 1
        CASE ELSE
            fact& = n * fact&(n - 1)
    END SELECT
END FUNCTION
