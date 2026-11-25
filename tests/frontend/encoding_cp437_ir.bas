' TEST: ir
'$ENCODING:CP437
' comment
CONST frame = "ÉÍÍ»"
PRINT frame; "‚"; "û"; ""; "a	b"
LINE INPUT "¨nombre? "; n$
SELECT CASE n$
    CASE "¤": PRINT 1
END SELECT
READ a$, b$
DATA caf‚, "na‹ve"
' ‚t‚
PRINT 1 REM x
