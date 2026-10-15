$CONSOLE:ONLY
' DECLARE DYNAMIC LIBRARY with a name the DLL does not export: error 260 when the program starts,
' before its first statement (and before ON ERROR can be set).
ON ERROR GOTO handler
DECLARE DYNAMIC LIBRARY "./v37dll"
    FUNCTION d_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    FUNCTION d_none& (BYVAL a AS LONG)
END DECLARE
PRINT "start"
PRINT d_add&(1, 2)
SYSTEM
handler:
PRINT "handled"; ERR
RESUME NEXT
