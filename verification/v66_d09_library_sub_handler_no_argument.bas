$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-events row): a SUB of a DECLARE LIBRARY block with no parameter (v66_d_lib.h) as
' the handler of ON TIMER, beside v66_d01 (srand, with one); the library's count shows whether it ran.
DECLARE LIBRARY "./v66_d_lib"
    SUB v66_tick
    FUNCTION v66_get& ()
END DECLARE
ON TIMER(0.05) v66_tick
TIMER ON
_DELAY 0.5
TIMER OFF
PRINT "ran:"; v66_get& > 0
SYSTEM
