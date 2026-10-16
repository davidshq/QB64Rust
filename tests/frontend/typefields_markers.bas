' TEST: check-fail
$CONSOLE:ONLY
' The _DYNAMIC and _STATIC member markers the old compiler rejects (m3-typefields design D2, task 2.2): without
' $UNSTABLE:TYPEFIELDS above the block (v24_x204, x205), between AS and the type (v35_x05), both markers or one
' twice (x06, x07), on a member that is no array (x08, x09), also in an AS list, before an AS list (x60). Each is
' an error of the program, none "not supported yet"; the messages are new.
TYPE before
    v(0 TO 3) _DYNAMIC AS LONG
END TYPE
TYPE beforestatic
    _STATIC v(3) AS LONG
END TYPE
$UNSTABLE:TYPEFIELDS
TYPE afteras
    AS _DYNAMIC LONG v(1)
END TYPE
TYPE both
    v(1) _DYNAMIC _STATIC AS LONG
END TYPE
TYPE twice
    v(1) _DYNAMIC _DYNAMIC AS LONG
END TYPE
TYPE frontandback
    _STATIC v(1) AS LONG _STATIC
END TYPE
TYPE scalar
    n _DYNAMIC AS LONG
END TYPE
TYPE scalarstatic
    n AS LONG _STATIC
END TYPE
TYPE listscalar
    AS LONG v(1), _STATIC n
END TYPE
TYPE listboth
    AS LONG _DYNAMIC v(1) _STATIC
END TYPE
TYPE beforelist
    _STATIC AS LONG v(1), w(2)
END TYPE
SYSTEM
