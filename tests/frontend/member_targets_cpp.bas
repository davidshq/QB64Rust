' TEST: cpp
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Targets reached through a member with an index on the way (m3-typefields task 4.2, verification\v35_x62): a
' member of an element, an element of a fixed and of a dynamic member array, as targets of READ, INPUT from a
' file and from the console, LINE INPUT and the MID$ statement, numbers and strings. Stored by the rule of their
' place: a number read before the indexes and not stored after an index outside (QB64PE-DIVERGENCES.md D-004), a string
' after them; console INPUT evaluates every target's indexes after its prompt and before the line is read; MID$
' the index before the value.
TYPE pt
    x AS LONG
    s AS STRING
    v(2) AS LONG
    w(2) AS STRING
    d(1) _DYNAMIC AS LONG
END TYPE
DIM a(2) AS pt, t AS pt, i AS LONG
REDIM a(1).d(1) AS LONG
DATA 1, "b", 3, 4
READ a(i).x, a(i + 1).s, t.v(i), a(1).d(i)
OPEN "member_targets.txt" FOR INPUT AS #1
INPUT #1, a(i).x, t.w(i)
LINE INPUT #1, a(i).w(1)
CLOSE #1
INPUT "? ", t.v(i), a(i).s
MID$(a(i).s, 1) = "x"
MID$(t.w(i), 2, 1) = "y"
SYSTEM
