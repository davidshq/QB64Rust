' TEST: check-fail
$CONSOLE:ONLY
' Names, declarations and arrays (m4-names-arrays): what stays marked or is an error. A second TYPE block of a
' name, wherever it stands, is "Name already in use" (design D4, verification\v48_x05). LEN of a whole array of
' variable-length strings, of _BIT and of a TYPE with a STRING member are errors (D5, v48_x01-x03); of a whole
' dynamic member array, marked; of a whole member array of strings (v48_x30), accepted since m4-memory-records
' (string_members_typed.bas); of a name that is no array, accepted since m4-names-implicit-arrays (v48_x14). An
' array without AS beside one
' listed by
' COMMON ... AS (design D2: not measured) is marked; the marked declaration stands last (the follow-on rule).
$UNSTABLE:TYPEFIELDS
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
TYPE own
    _DYNAMIC m(1 TO 4) AS LONG
END TYPE
TYPE strs
    s(3) AS STRING
END TYPE
DIM s(3) AS STRING, bits(3) AS _BIT, sa(3) AS ts, o AS own, ss AS strs
PRINT LEN(s())
PRINT LEN(bits())
PRINT LEN(sa())
PRINT LEN(o.m())
PRINT LEN(nothing())
TYPE twice
    x AS LONG
END TYPE
IF 1 THEN
    TYPE twice
        x AS INTEGER
    END TYPE
END IF
COMMON SHARED g() AS DOUBLE
DIM g(3)
