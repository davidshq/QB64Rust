' TEST: parse-ok
$CONSOLE:ONLY
' I/O statements without a template (m2-parser-breadth task 7.4, design D5): every form parses; sema marks them
' "not supported yet". The old compiler accepts this file (`qb64pe.exe -z`, 2026-10-07).
DIM a(3) AS STRING
PRINT #1, "x"; 2
PRINT #f + 1, USING "##.#"; 3.14
PRINT USING "&"; "s"
? USING "#"; 1, 2
LPRINT "x", 1
LPRINT USING "##"; 5
WRITE #1, "a", 2
WRITE 1, 2
WRITE
INPUT x
INPUT ; "Name"; n$
INPUT "Age", age, b$
INPUT #1, a(1), q
LINE INPUT l$
LINE INPUT ; "Prompt: "; l$
LINE INPUT #2, a(2)
CLOSE
CLOSE #1, 2, #f
FIELD #1, 10 AS a$, 20 AS b$
FIELD 2, 5 AS c$
LSET a$ = "left"
RSET a(3) = b$ + "x"
SWAP a(1), a(2)
SWAP x, q
SYSTEM
