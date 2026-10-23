$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (09_line_input)
DIM a(2) AS STRING
OPEN "v54_x09.tmp" FOR OUTPUT AS #1
PRINT #1, "line one"
CLOSE #1
OPEN "v54_x09.tmp" FOR INPUT AS #1
LINE INPUT #1, a()
CLOSE #1
KILL "v54_x09.tmp"
PRINT "["; a(0); "]"
SYSTEM
