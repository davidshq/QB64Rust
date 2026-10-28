$CONSOLE:ONLY
' Verification (from the review of libqb-fixes, D-048): how LINE INPUT and INPUT read a console input redirected
' from a file with CR LF line ends. v63_a6_line_input_crlf.stdin holds "ab" CR LF, "c,d" CR LF, "e" CR CR LF, then
' "z" CR LF that is never read. D-048 reads INPUT$ of such an input in the text mode INPUT reads it in; this program
' stops before the end of its input, where the old compiler's programs wait for ever.
LINE INPUT a$
PRINT LEN(a$);
FOR i = 1 TO LEN(a$): PRINT ASC(a$, i);: NEXT
PRINT
INPUT b$, c$
PRINT "<"; b$; "><"; c$; ">"
LINE INPUT e$
PRINT LEN(e$);
FOR i = 1 TO LEN(e$): PRINT ASC(e$, i);: NEXT
PRINT
PRINT "done"
SYSTEM
