$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): ERASE of a plain dotted array with blanks.
DIM p.q(3)
p.q(1) = 5
ERASE p . q
PRINT p.q(1)
SYSTEM
