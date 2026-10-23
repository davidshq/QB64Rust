' TEST: check-fail
$CONSOLE:ONLY
' What stays "not supported yet" around arrays in procedures after m2-defxxx-common-arrays task 6.1, each by its
' own message (measured, verification\v24_x121, x126, x272, x280): STATIC with a bound that is not one number or
' constant (the array is allocated in the main module's code: with a parameter the old compiler's C++ build
' fails, with 2 + 3 it works), STATIC beside a shared array of the main module (the old program ends when it
' starts: "Duplicate definition"). An array without a type beside one listed with AS (u4 before) and REDIM of a
' name without parentheses (u5) compile since m4-names-arrays: names_arrays_typed.bas.
DIM SHARED sh(3) AS LONG
SUB u1 (n AS LONG)
    STATIC v(n) AS LONG
END SUB
SUB u2
    STATIC v(2 + 3) AS LONG
END SUB
SUB u3
    STATIC sh(5) AS LONG
END SUB
