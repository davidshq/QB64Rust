' TEST: ir
'$ENCODING:UTF8
' 日本語のコメント …
CONST frame = "╔══╗"
PRINT frame; "é"; "√"; "♥⌂"; "a	b"
LINE INPUT "¿nombre? "; n$
SELECT CASE n$
    CASE "ñ": PRINT 1
END SELECT
READ a$, b$
DATA café, "naïve"
' été
PRINT 1 REM 日
