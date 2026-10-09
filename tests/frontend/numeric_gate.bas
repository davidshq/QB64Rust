' TEST: check-fail
$CONSOLE:ONLY
' Values of the new numeric types compute since m2-numeric-types task group 5 (`numeric_ops_cpp.bas`,
' `mixed_signedness.bas`); the uses that task group 8 brings are "not supported yet": a constant used with another
' suffix, a FUNCTION result, a `FOR` variable, a `SELECT CASE` selector, an argument of a special-cased built-in, a
' variable passed to a parameter of the other signedness (by reference, measured; in parentheses it is a copy and is
' accepted), and arrays and `TYPE` members of them. The declarations themselves are accepted
' (`numeric_decls.bas`), so the follow-on rule does not start and the real error after these lines is reported (the
' array and the member are declarations marked "not supported yet": they start it, so they come last).
DIM b AS _UNSIGNED _BYTE
CONST c~%% = 5, big = 4294967295
PRINT c~%%; big
PRINT big~&
PRINT f%%
FOR u~& = 1 TO 2: NEXT
SELECT CASE b
END SELECT
PRINT ABS(b)
PRINT HEX$(300~%%)
DIM s AS _BYTE
takes (s)
takes s
x = "real error"
DIM a(3) AS _BYTE
TYPE t
    m AS _UNSIGNED LONG
END TYPE
SYSTEM

FUNCTION f%%
END FUNCTION

SUB takes (p AS _UNSIGNED _BYTE)
END SUB
