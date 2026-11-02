' TEST: check-fail
$CONSOLE:ONLY
' The 40-character limit on names (TODO.md row 1; measured, verification\v66_e01-e18): a word of code of more
' than 40 characters, without its suffix and trailing _, is an error whatever it names, each time it stands; each
' part of a dotted name is a word, and a dotted variable, CONST or SUB name of more than 40 characters with its
' dots is an error too (e13, e17, e18). The TYPE below is still declared, so its use gives no second error.
abcdefghijabcdefghijabcdefghijabcdefghijk = 1
DIM abcdefghijabcdefghijabcdefghijabcdefghijl(3)
CONST abcdefghijabcdefghijabcdefghijabcdefghijm = 2
TYPE t1
    abcdefghijabcdefghijabcdefghijabcdefghijn AS LONG
END TYPE
DIM t AS t1
PRINT LEN(t)
abcdefghijabcdefghijabcdefghij.abcdefghijabcdefghijabcdefghijabcdefghijo = 3
abcdefghijabcdefghijabcdefghij.abcdefghijabcdefghijabcdefghij = 4
CONST abcdefghijabcdefghijabcdefghij.abcdefghijabcdefghijabcdefghik = 5
GOTO abcdefghijabcdefghijabcdefghijabcdefghijp
abcdefghijabcdefghijabcdefghijabcdefghijp:
s 1
SYSTEM

SUB s (abcdefghijabcdefghijabcdefghijabcdefghijq AS LONG)
END SUB

SUB abcdefghijabcdefghijabcdefghij.abcdefghijabcdefghijabcdefghil
END SUB
