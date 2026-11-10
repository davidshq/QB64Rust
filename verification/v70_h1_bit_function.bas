$CONSOLE:ONLY
' Verification (study 44, section 2 item 8): a FUNCTION declared with the _BIT suffix and called without it, as
' in the Museum's aouwt\QB-WM\wm.bas (`FUNCTION mouseIsOver` (win AS INTEGER)`).
FOR i = 0 TO 1
    IF m(i) THEN PRINT "yes"; i ELSE PRINT "no"; i
NEXT
PRINT m`(1)
SYSTEM

FUNCTION m` (w AS INTEGER)
    m` = w > 0
END FUNCTION
