$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): INPUT$ on a file (found by the table test of design D10): n characters from a
' file open FOR INPUT, with and without the # before the file number, 0 characters, across a line end, past the end
' of the file (error 62), a negative count (error 52), from a BINARY file, from a file open FOR OUTPUT (error 75),
' from a file not open (error 52), and the result's type.
ON ERROR GOTO handler
OPEN "v58_f16.tmp" FOR OUTPUT AS #1
PRINT #1, "abcdef"
PRINT #1, "ghi"
CLOSE #1
OPEN "v58_f16.tmp" FOR INPUT AS #1
PRINT INPUT$(3, #1); "|"; INPUT$(2, 1); "|"; LEN(INPUT$(0, #1)); "|"; INPUT$(4, #1); "|"
s$ = INPUT$(10, #1): PRINT "["; s$; "]"; LEN(s$)
s$ = INPUT$(-1, #1): PRINT LEN(s$)
CLOSE #1
OPEN "v58_f16.tmp" FOR BINARY AS #2
PRINT INPUT$(8, #2); "|"; LEN(INPUT$(20, #2))
CLOSE #2
OPEN "v58_f16.tmp" FOR OUTPUT AS #3
s$ = INPUT$(1, #3): PRINT LEN(s$)
CLOSE #3
s$ = INPUT$(1, #4): PRINT LEN(s$)
n = 2: f = 2: OPEN "v58_f16.tmp" FOR INPUT AS #f
PRINT INPUT$(n, f); INPUT$(n + 1, #f)
CLOSE #f
KILL "v58_f16.tmp"
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT
