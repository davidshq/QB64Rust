DECLARE SUB topbar ()
DECLARE SUB ships ()
DECLARE SUB cannon ()
$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): a line that is the name of a SUB and a colon, as Civil War
' Strategy has it. With no jump to the name anywhere it is a call of the SUB (topbar); with a jump it is a
' label, and CALL still calls the SUB (ships, cannon).
DIM SHARED n AS INTEGER
CALL topbar
PRINT "main"
GOSUB part
CALL navy
SYSTEM
part:
PRINT "part"
RETURN
topbar:
PRINT "after the line topbar: in the main module"
RETURN

SUB topbar
    PRINT "sub topbar"
END SUB

SUB navy
    ships:
    n = n + 1
    PRINT "navy"; n
    IF n < 2 GOTO ships
    CALL ships
    cannon:
    n = n + 1
    IF n < 5 THEN GOTO cannon
    CALL cannon
    PRINT "navy ends"; n
END SUB

SUB ships
    PRINT "sub ships"
END SUB

SUB cannon
    PRINT "sub cannon"
END SUB
