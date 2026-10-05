' TEST: parse-ok
$CONSOLE:ONLY
' TYPE blocks (every field form found in the inputs) and DECLARE LIBRARY blocks (m2-parser-breadth task 6.3;
' measured in verification\v16_m4_type_forms, v16_m4_type_in_sub, v16_m4_declare_library_forms)
TYPE inner
    v AS LONG
END TYPE
TYPE outer
    ' a comment
    a AS INTEGER
    AS LONG b, c
    s AS STRING * 4
    AS STRING * 2 t
    u AS _UNSIGNED _BYTE
    n AS inner

    arr(1 TO 3) AS LONG
    m(0 TO 1, 0 TO 2) AS STRING * 8
    AS LONG values(0 TO 1) _DYNAMIC, scalar
    items(0 TO 1) _DYNAMIC AS inner
    _STATIC text(9) AS STRING * 23
END TYPE
DECLARE LIBRARY
    FUNCTION toupper& (BYVAL c AS LONG)
    FUNCTION up2& ALIAS "toupper" (BYVAL c AS LONG)
    FUNCTION up3~& ALIAS toupper (BYVAL c AS _UNSIGNED LONG)
    ' a comment
    SUB srand (BYVAL seed AS _UNSIGNED LONG)
END DECLARE
DECLARE DYNAMIC LIBRARY "lib"
    FUNCTION f& ()
END DECLARE
DECLARE CUSTOMTYPE LIBRARY
END DECLARE
SUB s
    TYPE t
        a AS LONG
    END TYPE
END SUB
