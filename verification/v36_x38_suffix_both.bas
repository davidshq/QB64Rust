$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (38_suffix_both)
DIM a%(2), b%(2)
a%(0) = 1: a%(1) = 2: b%(2) = 9
b%() = a%()
PRINT b%(0); b%(1); b%(2)
PRINT "ok"
SYSTEM
