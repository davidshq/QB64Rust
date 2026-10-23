$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): file PUT of a whole array of strings of a name that is no array
' (uns$()), where no statement above names it.
OPEN "v56_e2_put_undeclared_string_array.tmp" FOR BINARY AS #1
PUT #1, 1, uns$()
PRINT LOF(1); SEEK(1); UBOUND(uns$)
CLOSE #1
KILL "v56_e2_put_undeclared_string_array.tmp"
SYSTEM
