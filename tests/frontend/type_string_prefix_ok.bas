' TEST: check-ok
$CONSOLE:ONLY
' A TYPE whose name starts with STRING is a type in every AS clause (m4-upstream-rejections, task 2.3): the old
' compiler looks the user types up before the STRING prefix (dim2, qb64pe.bas 17860; typname2typ, type.bas 559),
' so the "Expected STRING * number/constant" rule of type_names_errors.bas does not reach it. Not yet recorded
' with qb64pe.exe (tasks.md, "Workflow follow-up").
TYPE StringList
    n AS INTEGER
END TYPE
TYPE Strings
    s AS STRING * 4
END TYPE
DIM a AS StringList
DIM SHARED b AS Strings
REDIM c(1 TO 2) AS StringList
a.n = 3
b.s = "abcd"
c(1).n = 4
show a, c()
PRINT a.n; b.s; c(1).n
SUB show (x AS StringList, y() AS StringList)
    SHARED b AS Strings
    STATIC d AS Strings
    d.s = "wxyz"
    PRINT x.n; y(1).n; b.s; d.s
END SUB
