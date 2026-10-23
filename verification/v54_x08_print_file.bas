$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (08_print_file)
DIM a(2) AS LONG
a(0) = 4
OPEN "v54_x08.tmp" FOR OUTPUT AS #1
PRINT #1, a()
CLOSE #1
OPEN "v54_x08.tmp" FOR INPUT AS #1
LINE INPUT #1, l$
CLOSE #1
KILL "v54_x08.tmp"
PRINT "["; l$; "]"
SYSTEM
