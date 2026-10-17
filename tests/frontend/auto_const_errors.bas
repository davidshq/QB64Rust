' TEST: check-fail
$CONSOLE:ONLY
' The constants of QB64pe's auto-included file rejected (m2-console-screenhide task 5.2; each form one program of
' verification\v26_x115-x126, where the old compiler rejects it too): a name of the file is a reserved name, so a
' CONST, a DIM, a SUB, an assignment, FOR, INPUT, READ and SWAP with it are errors, with the same value too; a
' numeric constant with `$` is a type mismatch; LEN of a numeric constant is an error. Left "not supported yet",
' with the reason: a constant whose value needs EXP (_E; also as a string length; _STR_CRLF and _CHR_CR compile
' since m3-errorline-chr-const), one whose number the CONST evaluator does not read yet (_DOUBLE_MAX, _FLOAT_MAX,
' _INTEGER64_MIN, _UINTEGER64_MAX: the old compiler's values for them are in v26_x114), a constant with an index.
' The functions of the file included after the last line are compiled since m3-network-embed (url_errors).
DIM n AS LONG, d AS DOUBLE, s AS STRING
CONST _TRUE = 1
CONST _ONE_MB = 1048576
DIM _TRUE AS LONG
_TRUE = 5
n = LEN(_TRUE$)
FOR _TRUE = 1 TO 2: NEXT
INPUT _TRUE
READ _TRUE
SWAP n, _TRUE
CONST _MYOWN = 1
n = LEN(_BYTE_MAX)
_STR_EMPTY = "a"
n = _TRUE(1)
s = _STR_CRLF
s = _CHR_CR
d = _E
d = _DOUBLE_MAX
d = _FLOAT_MAX
n = _INTEGER64_MIN
n = _UINTEGER64_MAX
CONST k = _CHR_CR
DIM t AS STRING * _E
' s = _WhatIsMyIP stood here: an error since m3-network-embed (url_errors)
SYSTEM

SUB _FALSE
END SUB
