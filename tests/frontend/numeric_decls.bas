' TEST: typed
$CONSOLE:ONLY
' The declarations of the new numeric types (m2-numeric-types task 4.1, measured verification\v21_a_decls and
' v21_a_suffixes): every AS spelling, the suffixes on DIMmed and implicit names, DIM SHARED, STATIC and SHARED,
' FUNCTION names and parameters, and LEN of each (1, 1, 2, 2, 4, 4, 8, 8, 8, 8). `_UNSIGNED STRING` is a STRING;
' one name with each suffix is a variable of its own. Most declarations stand in a SUB, whose variables the dump
' lists with their types. Written when their values were still "not supported yet" (task group 5 brought them), so
' nothing here loads or stores one; LEN of a variable is its size, never a value.
DIM SHARED m%%, n~%%, o~%, p~&, q~&&, r%&, t~%&, u`, v~`, w`3, x~`64
decls
show
SYSTEM

SUB decls
    DIM a AS _BYTE, b AS _UNSIGNED _BYTE, c AS _UNSIGNED INTEGER, d AS _UNSIGNED LONG
    DIM e AS _UNSIGNED _INTEGER64, f AS _OFFSET, g AS _UNSIGNED _OFFSET
    DIM h AS _BIT, i AS _UNSIGNED _BIT, j AS _BIT * 7, k AS _UNSIGNED _BIT*40, s AS _UNSIGNED STRING
    DIM bb%%, cc~`5
    STATIC st AS _UNSIGNED LONG
    PRINT LEN(a); LEN(b); LEN(c); LEN(d); LEN(e); LEN(f); LEN(g); LEN(s); LEN(st)
    PRINT LEN(nm%%); LEN(nm~%%); LEN(nm%); LEN(nm~%); LEN(nm&); LEN(nm~&); LEN(nm&&); LEN(nm~&&); LEN(nm%&); LEN(nm~%&)
END SUB

SUB show
    SHARED d AS _UNSIGNED LONG, n~%%
    PRINT LEN(d); LEN(n~%%)
END SUB

FUNCTION never~%% (p1 AS _UNSIGNED LONG, p2%&)
END FUNCTION
