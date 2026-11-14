$CONSOLE:ONLY
OPEN "input-str_1.tmp" FOR OUTPUT AS #1
PRINT #1, "abcdef"
CLOSE #1
OPEN "input-str_1.tmp" FOR INPUT AS #1
PRINT INPUT$(3, #1); "|"; INPUT$(2, 1)
CLOSE #1
KILL "input-str_1.tmp"
END
