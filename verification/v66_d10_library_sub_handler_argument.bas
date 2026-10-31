$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-events row): a SUB of a DECLARE LIBRARY block with a BYVAL LONG parameter
' (v66_d_lib.h) as the handler of ON TIMER with an argument; the library's count shows whether it ran, and with 2.
DECLARE LIBRARY "./v66_d_lib"
    SUB v66_add (BYVAL n AS LONG)
    FUNCTION v66_get& ()
END DECLARE
ON TIMER(0.05) v66_add 2
TIMER ON
_DELAY 0.5
TIMER OFF
PRINT "ran:"; v66_get& > 0; "even:"; v66_get& MOD 2 = 0
SYSTEM
