' TEST: check-fail
$CONSOLE:ONLY
' sema marks every block kind "not supported yet" at its first word and still checks the statements inside (the
' strings stored in numbers below are real errors); `EXIT FOR` and the like are marked; `DEF FN` is a real error,
' as in the old compiler (m2-parser-breadth task 6.4)
FOR i = 1 TO 2
    a% = "for"
NEXT
DO
    b% = "do"
    EXIT DO
LOOP
WHILE 0
    c% = "while"
WEND
IF 1 THEN
    d% = "if"
ELSEIF 2 THEN
    e% = "elseif"
ELSE
    f% = "else"
END IF
IF 1 THEN g% = "then" ELSE h% = "else"
SELECT CASE 1
    CASE 1
        j% = "case"
END SELECT
TYPE t
    a AS LONG
END TYPE
DECLARE LIBRARY
    FUNCTION toupper& (BYVAL c AS LONG)
END DECLARE
DEF FNa (x) = x * 2
DEF FNb
    k% = "def"
END DEF
ON ERROR GOTO handler
FOR i = 1 TO 2
handler:
NEXT
