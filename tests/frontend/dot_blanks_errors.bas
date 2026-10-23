' TEST: check-fail
$CONSOLE:ONLY
' Blanks around the dot after a plain name that spells the name of a CONST or a FUNCTION (m4-devices-continue-paths
' design D2, found in its review): the old compiler drops the blanks before it reads the statement (qb64pe.bas
' 4196-4241), so each would be that constant or a call of that function. Not measured, so each is "not supported
' yet", never a new variable of that name
CONST c.k = 5
DIM x AS LONG
PRINT c . k
x = f . g
x = f . g + 1
FUNCTION f.g&
    f . g = 7
END FUNCTION
