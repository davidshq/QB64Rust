' TEST: check-fail
$CONSOLE:ONLY
' EXIT SELECT and EXIT CASE with no CASE open (m3-console-files task 3.1; verification\v30_x60, x61, x63, x64):
' outside any SELECT, between SELECT and its first CASE, after END SELECT, in a SUB called from a CASE. Each is an
' error that is not "not supported yet"
DIM n AS LONG
EXIT SELECT
EXIT CASE
SELECT CASE n
    EXIT SELECT
    CASE 1
END SELECT
SELECT CASE n
    CASE 1
END SELECT
EXIT SELECT
SELECT CASE n
    CASE 1
        t
END SELECT
FOR n = 1 TO 2
    EXIT SELECT
NEXT
SUB t
    EXIT SELECT
    EXIT CASE
END SUB
