$CONSOLE:ONLY
' Verification (m3-window-output, task 4.4): PALETTE USING, which the old compiler reads by hand, outside the
' template of PALETTE; as code that is not reached.
DIM a(15) AS INTEGER, b(15) AS LONG
PRINT "ok"
SYSTEM
PALETTE USING a(0)
PALETTE USING a
PALETTE USING b(2)
