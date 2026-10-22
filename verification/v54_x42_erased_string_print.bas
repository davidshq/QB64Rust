$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, review): does the old compiler accept this, and what does it do? (42_erased_string_print)
REDIM s(2) AS STRING
s(0) = "abc"
PRINT "["; s(); "]"
ERASE s
PRINT "["; s(); "]"
PRINT "end"
SYSTEM
