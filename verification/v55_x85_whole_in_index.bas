$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, review): does the old compiler accept this, and what does it do? (x85_whole_in_index)
DIM a(100)
a(LEN(w())) = 1
DATA 3
READ a(LEN(v#()))
PRINT a(44); a(88)
SYSTEM
