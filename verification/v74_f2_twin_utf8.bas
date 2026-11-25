'$ENCODING:UTF8
$CONSOLE:ONLY
' Verification (utf8-source, task 2.6): one program saved as UTF-8. Its strings, DATA items, a CONST
' and a SELECT CASE hold box-drawing characters, accents, a root sign and the heart of byte 3.
CONST frame = "╔══╗"
PRINT LEN(frame); ASC(frame); ASC(frame, 2); ASC(frame, 4)
PRINT ASC("é"); ASC("√"); ASC("♥")
READ a$, b$
PRINT LEN(a$); ASC(a$, 4); LEN(b$); ASC(b$, 3)
SELECT CASE b$
    CASE "naïve": PRINT "matched"
    CASE ELSE: PRINT "not matched"
END SELECT
s$ = "─│"
FOR i = 1 TO LEN(s$): PRINT ASC(s$, i);: NEXT: PRINT
DATA café, "naïve"
SYSTEM
