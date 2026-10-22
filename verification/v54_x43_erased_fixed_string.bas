$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, review): does the old compiler accept this, and what does it do? (43_erased_fixed_string)
REDIM f(2) AS STRING * 20
f(0) = "abc"
PRINT "["; f(); "]"
ERASE f
PRINT "["; f(); "]"
PRINT "end"
SYSTEM
