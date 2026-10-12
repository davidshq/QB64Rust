$CONSOLE:ONLY
' Slice program (m2-core-builtins, D7, D9): SELECT CASE and SELECT EVERYCASE as measured in verification\v20_h_select:
' when the selector is read, items converted to the selector's type, string items, IS with each operator, errors
' in the selector and in items (the handler prints ERR and resumes next), jumps out of and into a CASE body, and a
' recursive FUNCTION across a SELECT (DIVERGENCES-QB45.md Q-002: one static copy per SELECT). No PRINT comma.
DIM SHARED v AS INTEGER
DIM SHARED a(3) AS INTEGER
TYPE t
    m AS INTEGER
END TYPE
DIM SHARED u AS t
ON ERROR GOTO h

PRINT "-- selector read once"
SELECT CASE noisy
    CASE 1: PRINT "one"
    CASE 2: PRINT "two"
    CASE 3: PRINT "three"
END SELECT

PRINT "-- a plain variable is read at each test; an element and a member once"
v = 1
SELECT CASE v
    CASE setv: PRINT "case setv"
    CASE 2: PRINT "case 2"
    CASE ELSE: PRINT "else"
END SELECT
a(1) = 1
SELECT CASE a(1)
    CASE seta: PRINT "case seta"
    CASE 2: PRINT "case 2"
    CASE ELSE: PRINT "else"
END SELECT
u.m = 1
SELECT CASE u.m
    CASE setm: PRINT "case setm"
    CASE 2: PRINT "case 2"
    CASE ELSE: PRINT "else"
END SELECT

PRINT "-- items converted to the selector's type"
DIM i AS INTEGER
i = 2
SELECT CASE i
    CASE 2.4: PRINT "2.4 matches 2"
    CASE ELSE: PRINT "else"
END SELECT
SELECT CASE i
    CASE 1.5: PRINT "1.5 matches 2"
    CASE ELSE: PRINT "else"
END SELECT
SELECT CASE i
    CASE 2.5 TO 3: PRINT "2.5 TO 3 matches 2"
    CASE ELSE: PRINT "else"
END SELECT
SELECT CASE i
    CASE IS > 1.9: PRINT "IS > 1.9"
    CASE ELSE: PRINT "IS > 1.9 does not match 2"
END SELECT
DIM f AS SINGLE
f = 2.5
SELECT CASE f
    CASE 2: PRINT "2"
    CASE 3: PRINT "3"
    CASE 2 TO 3: PRINT "2.5 in 2 TO 3"
    CASE ELSE: PRINT "else"
END SELECT
DIM d AS DOUBLE
d = 0.1
SELECT CASE d
    CASE 0.1: PRINT "0.1 matches DOUBLE 0.1"
    CASE ELSE: PRINT "0.1 does not match DOUBLE 0.1"
END SELECT
f = 0.1
SELECT CASE f
    CASE 0.1#: PRINT "0.1# matches SINGLE 0.1"
    CASE ELSE: PRINT "0.1# does not match SINGLE 0.1"
END SELECT
DIM q AS _INTEGER64
q = 4294967298
SELECT CASE q
    CASE 2: PRINT "4294967298 matches 2"
    CASE 4294967298: PRINT "4294967298"
END SELECT
SELECT CASE i + 1
    CASE 3: PRINT "i + 1 is 3"
END SELECT

PRINT "-- string selectors"
DIM s AS STRING
s = "dog"
SELECT CASE s
    CASE "a" TO "m": PRINT "a TO m"
    CASE ELSE: PRINT "else"
END SELECT
SELECT CASE s
    CASE IS < "b": PRINT "IS < b"
    CASE IS >= "dog": PRINT "IS >= dog"
END SELECT
SELECT CASE s
    CASE "cat", "dog": PRINT "cat, dog"
END SELECT
SELECT CASE s + "s"
    CASE "dogs": PRINT "dogs"
END SELECT
SELECT CASE LEFT$(s, 1)
    CASE "d": PRINT "d"
END SELECT

PRINT "-- IS with each operator (EVERYCASE), selector 5"
i = 5
SELECT EVERYCASE i
    CASE IS = 5: PRINT "= 5"
    CASE IS <> 5: PRINT "<> 5"
    CASE IS < 6: PRINT "< 6"
    CASE IS > 4: PRINT "> 4"
    CASE IS <= 5: PRINT "<= 5"
    CASE IS >= 6: PRINT ">= 6"
END SELECT

PRINT "-- items of each kind"
FOR k = 1 TO 4
    IF k = 1 THEN x = 3
    IF k = 2 THEN x = 5
    IF k = 3 THEN x = 11
    IF k = 4 THEN x = 8
    SELECT CASE x
        CASE 1, 3: PRINT "first"
        CASE 4 TO 6: PRINT "second"
        CASE IS > 10: PRINT "third"
        CASE ELSE: PRINT "else"
    END SELECT
NEXT
SELECT CASE 7
    CASE 9 TO 1: PRINT "9 TO 1 matched 7"
    CASE ELSE: PRINT "9 TO 1 did not match 7"
END SELECT
SELECT CASE 9
    CASE 1: PRINT "one"
END SELECT
PRINT "no match, no ELSE"

PRINT "-- EVERYCASE"
x = 5
SELECT EVERYCASE x
    CASE IS > 1: PRINT "> 1"
    CASE IS > 2: PRINT "> 2"
    CASE ELSE: PRINT "else"
END SELECT
SELECT EVERYCASE x
    CASE IS > 9: PRINT "> 9"
    CASE ELSE: PRINT "else"
END SELECT
SELECT EVERYCASE x
    CASE 5: PRINT "5": x = 6
    CASE 6: PRINT "6 after the body changed x"
END SELECT
everyp 1
everyp 3

PRINT "-- errors in the selector and in items"
SELECT CASE ASC("")
    CASE 0: PRINT "case 0"
    CASE 1: PRINT "case 1"
    CASE ELSE: PRINT "else"
END SELECT
SELECT EVERYCASE ASC("")
    CASE 0: PRINT "case 0"
    CASE ELSE: PRINT "else"
END SELECT
x = 5
SELECT CASE x
    CASE ASC(""): PRINT "case ASC(empty) for 5"
    CASE ELSE: PRINT "else for 5"
END SELECT
SELECT CASE x
    CASE 1, ASC(""), 2: PRINT "case 1, ASC(empty), 2 for 5"
    CASE ELSE: PRINT "else for 5 (list)"
END SELECT
SELECT CASE x
    CASE ASC("") TO 9: PRINT "case ASC(empty) TO 9 for 5"
    CASE ELSE: PRINT "else for 5 (range)"
END SELECT
SELECT CASE x
    CASE IS > ASC(""): PRINT "case IS > ASC(empty) for 5"
    CASE ELSE: PRINT "else for 5 (IS)"
END SELECT
SELECT CASE a(9)
    CASE 0: PRINT "bad index: element 0's value"
    CASE ELSE: PRINT "else"
END SELECT

PRINT "-- jumps"
x = 1
SELECT CASE x
    CASE 1
        PRINT "in case 1, GOTO out"
        GOTO out1
        PRINT "not printed"
    CASE ELSE
        PRINT "else"
END SELECT
out1:
PRINT "out1"
GOTO inside
SELECT CASE 1
    CASE 2
        PRINT "case 2 head"
inside:
        PRINT "inside case 2"
    CASE 3
        PRINT "case 3"
END SELECT
PRINT "after jump in"
exits
SELECT CASE 1
END SELECT
PRINT "empty SELECT ran"
FOR k = 1 TO 3
    SELECT CASE k
        CASE 2: EXIT FOR
    END SELECT
    PRINT "k ="; k
NEXT
PRINT "EXIT FOR from a CASE at k ="; k

PRINT "-- recursion (Q-002)"
PRINT "rec(1) ="; rec(1)
PRINT "rec(3) ="; rec(3)
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT

FUNCTION noisy
    PRINT "noisy called"
    noisy = 3
END FUNCTION

FUNCTION setv
    v = 2
    setv = 0
END FUNCTION

FUNCTION seta
    a(1) = 2
    seta = 0
END FUNCTION

FUNCTION setm
    u.m = 2
    setm = 0
END FUNCTION

SUB exits
    SELECT CASE 1
        CASE 1
            PRINT "EXIT SUB from a CASE"
            EXIT SUB
    END SELECT
    PRINT "not printed"
END SUB

SUB everyp (n)
    SELECT EVERYCASE n * 2
        CASE 2: PRINT "everyp 2"
        CASE IS < 5: PRINT "everyp < 5"
        CASE ELSE: PRINT "everyp else"
    END SELECT
END SUB

FUNCTION rec (n)
    SELECT CASE n * 10
        CASE probe(n): rec = -1
        CASE 10: rec = 1
        CASE 20: rec = 2
        CASE 30: rec = 3
        CASE ELSE: rec = 99
    END SELECT
END FUNCTION

FUNCTION probe (n)
    IF n = 1 THEN dummy = rec(2)
    probe = -5
END FUNCTION
