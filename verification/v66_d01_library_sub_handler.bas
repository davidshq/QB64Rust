$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-events row): a SUB of a DECLARE LIBRARY block (the C library's srand) as the handler of ON TIMER; rand after it shows whether it ran with its argument.
DECLARE LIBRARY
    SUB srand (BYVAL s AS _UNSIGNED LONG)
    FUNCTION rand& ()
END DECLARE
ON TIMER(0.05) srand 5
TIMER ON
_DELAY 0.5
TIMER OFF
r1& = rand
srand 5
r2& = rand
srand 6
r3& = rand
PRINT "after the handler:"; r1& = r2&; r1& = r3&
SYSTEM
