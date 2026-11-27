'$ENCODING:CP437
$CONSOLE:ONLY
' Verification (utf8-source, task 2.6): one program saved as code page 437. Its strings, DATA items, a CONST
' and a SELECT CASE hold box-drawing characters, accents, a root sign and the heart of byte 3.
CONST frame = "ÉÍÍ»"
PRINT LEN(frame); ASC(frame); ASC(frame, 2); ASC(frame, 4)
PRINT ASC("‚"); ASC("û"); ASC("")
READ a$, b$
PRINT LEN(a$); ASC(a$, 4); LEN(b$); ASC(b$, 3)
SELECT CASE b$
    CASE "na‹ve": PRINT "matched"
    CASE ELSE: PRINT "not matched"
END SELECT
s$ = "Ä³"
FOR i = 1 TO LEN(s$): PRINT ASC(s$, i);: NEXT: PRINT
DATA caf‚, "na‹ve"
SYSTEM
