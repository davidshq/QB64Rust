$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-mem row): _OFFSET of a STRING member, against _OFFSET of its variable.
TYPE ts
    n AS LONG
    s AS STRING
END TYPE
DIM t AS ts
PRINT _OFFSET(t.s) - _OFFSET(t)
SYSTEM
