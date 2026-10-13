' TEST: ir
$CONSOLE:ONLY
' A line that is the name of a SUB and a colon (m3-get-put-bload design D1; measured, verification\v28_c_labels,
' v28_x71-x92). With no jump to the name anywhere in the program it is a call of the SUB, also with a statement
' after the colon (topbar, twice). With a jump in any body it is a label in every body, and CALL still calls the
' SUB (ships: the jump is in SUB navy, the main module's line is a label too). A SUB with a parameter and a
' FUNCTION are always labels (cannon, range). IF c THEN name and ON <event> name are calls, not jumps (flag)
DIM SHARED n AS INTEGER
topbar:
topbar: n = 1
ships:
cannon:
range:
flag:
IF n THEN flag
CALL navy
SYSTEM

SUB topbar
    n = n + 1
END SUB

SUB navy
    ships:
    n = n + 1
    IF n < 2 GOTO ships
    CALL ships
    topbar:
    ON n GOSUB away, away
    EXIT SUB
    away:
    RETURN
END SUB

SUB ships
END SUB

SUB cannon (k AS INTEGER)
END SUB

SUB flag
END SUB

FUNCTION range
    range = 1
END FUNCTION
