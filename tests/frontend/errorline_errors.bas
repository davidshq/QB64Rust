' TEST: check-fail
$CONSOLE:ONLY
' The error functions rejected (m3-errorline-chr-const; each form one program of verification\v43_x29-x42, where
' the old compiler rejects it too): an argument or empty parentheses where there is no slot, a statement, an
' assignment, a CONST, a type suffix, a string or two arguments to _ERRORMESSAGE$, a name without its dollar.
DIM s AS STRING
PRINT _ERRORLINE(1)
_ERRORLINE
s = _ERRORMESSAGE$("a")
s = _ERRORMESSAGE$(1, 2)
PRINT _ERRORMESSAGE(5)
s = _INCLERRORFILE$()
_ERRORLINE = 5
s = _ERRORMESSAGE$()
PRINT _ERRORLINE()
CONST c = _ERRORLINE
PRINT _ERRORLINE&
PRINT _INCLERRORFILE
SYSTEM
