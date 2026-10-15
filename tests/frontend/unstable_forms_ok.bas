' TEST: check-ok
$CONSOLE:ONLY
' The forms of $UNSTABLE the old compiler accepts (m3-typefields design D1, task 2.1; verification/v35_h, x13,
' x15, x47): any letter case, twice, a blank after the colon, in a $IF branch that is taken, after the first
' statement; MIDI and HTTP with their warning (CLI test unstable_stable_flag_warning).
PRINT 0
$unstable:typefields
$UNSTABLE: TYPEFIELDS
$IF 1 THEN
    $UNSTABLE:TYPEFIELDS
$END IF
$UNSTABLE:MIDI
$UNSTABLE:http
TYPE t
    v(1) _STATIC AS LONG
END TYPE
DIM x AS t
x.v(1) = 3
PRINT x.v(1)
SYSTEM
