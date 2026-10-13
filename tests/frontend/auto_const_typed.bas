' TEST: typed
$CONSOLE:ONLY
' The constants of QB64pe's auto-included file in the typed tree (m2-console-screenhide task 5.2, design D8;
' their values and types compared with the old compiler's by slice program s67_auto_constants): each becomes a
' constant of the main module where it is first named, and only then, so the list of constants below holds the
' ones this program names and the ones their expressions name (_ONE_KB for _ONE_MB; _MINS_IN_HOUR and
' _SECS_IN_MIN for _SECS_IN_HOUR). A use is a literal, as of any constant: _TRUE an _INTEGER64, a constant with a
' suffix a literal of that suffix, with another suffix a literal of the other (verification\v26_x114). In a SUB
' and a FUNCTION they are the same constants; a CONST may be computed from them; one is the length of a STRING * n
DIM l AS LONG, i AS INTEGER, d AS DOUBLE, s AS STRING
DIM fx AS STRING * _SIZE_OF_OFFSET
CONST k = _TRUE + 1
CONST m = _ONE_MB * 2
l = _TRUE
l = _FALSE
i = _BYTE_MAX
l = _LONG_MAX
l = _LONG_MAX%
d = _SINGLE_MAX
d = _DOUBLE_MIN_FRAC
l = _ONE_MB
l = _ONE_MB&&
i = _TRUE%
d = _TRUE#
s = _STR_EMPTY
IF _TRUE THEN l = _SECS_IN_HOUR
PRINT k; m; _BIT_MIN; _UBIT_MAX; _UINTEGER_MAX + 1
show _TRUE
PRINT twice(_ONE_KB)
SYSTEM

SUB show (a AS LONG)
    IF _TRUE THEN PRINT a; _SECS_IN_MIN * _MINS_IN_HOUR; _LOG_WARN
    CONST local = _ONE_KB + _TRUE
    PRINT local
END SUB

FUNCTION twice& (n AS LONG)
    twice& = n * 2 + _FALSE
END FUNCTION
