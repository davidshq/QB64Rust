' TEST: typed
$CONSOLE:ONLY
' Places in parentheses are the places inside them (m4-statements-metacommands, design D6; verification\v22_x103,
' v22_x84, v26_x128, v51_x34, v57_p01-p16): SWAP, CALL SWAP, READ, INPUT, LINE INPUT, LBOUND and UBOUND.
DIM x(3) AS INTEGER
DIM b(2 TO 5, 3) AS INTEGER
a& = 1: c& = 2
SWAP (a&), c&
SWAP a&, ((c&))
SWAP (x(1)), x(2)
CALL SWAP(a&, c&)
READ (q), ((r))
READ (x(1))
INPUT (n)
LINE INPUT (s$)
PRINT LBOUND((x)); UBOUND(((b)), 2)
DATA 1, 2, 3
