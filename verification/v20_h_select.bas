$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): SELECT CASE: when the selector is read, float items against integer
' selectors, string items, IS, EVERYCASE, errors in the selector and in items, jumps, recursion (Q-002).
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

PRINT "-- plain variable selector"
v = 1
SELECT CASE v
    CASE setv: PRINT "case setv"
    CASE 2: PRINT "case 2"
    CASE ELSE: PRINT "else"
END SELECT

PRINT "-- element selector"
a(1) = 1
SELECT CASE a(1)
    CASE seta: PRINT "case seta"
    CASE 2: PRINT "case 2"
    CASE ELSE: PRINT "else"
END SELECT

PRINT "-- member selector"
u.m = 1
SELECT CASE u.m
    CASE setm: PRINT "case setm"
    CASE 2: PRINT "case 2"
    CASE ELSE: PRINT "else"
END SELECT

PRINT "-- float items, integer selector 2"
DIM i AS INTEGER
i = 2
SELECT CASE i
    CASE 2.4: PRINT "2.4"
    CASE ELSE: PRINT "else"
END SELECT
SELECT CASE i
    CASE 1.5: PRINT "1.5"
    CASE ELSE: PRINT "else"
END SELECT
SELECT CASE i
    CASE 2.5 TO 3: PRINT "2.5 TO 3"
    CASE ELSE: PRINT "else"
END SELECT
SELECT CASE i
    CASE IS > 1.9: PRINT "IS > 1.9"
    CASE ELSE: PRINT "else"
END SELECT

PRINT "-- float selector 2.5, integer items"
DIM f AS SINGLE
f = 2.5
SELECT CASE f
    CASE 2: PRINT "2"
    CASE 3: PRINT "3"
    CASE 2 TO 3: PRINT "2 TO 3"
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

PRINT "-- string selector"
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

PRINT "-- IS with each operator, selector 5"
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

PRINT "-- no match, no ELSE"
SELECT CASE 9
    CASE 1: PRINT "one"
END SELECT
PRINT "after"

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

PRINT "-- error in the selector"
SELECT CASE ASC("")
    CASE 0: PRINT "case 0"
    CASE 1: PRINT "case 1"
    CASE ELSE: PRINT "else"
END SELECT
PRINT "-- error in the selector, EVERYCASE"
SELECT EVERYCASE ASC("")
    CASE 0: PRINT "case 0"
    CASE 1: PRINT "case 1"
    CASE ELSE: PRINT "else"
END SELECT
PRINT "-- error in an item"
x = 0
SELECT CASE x
    CASE ASC(""): PRINT "case ASC(empty)"
    CASE 0: PRINT "case 0"
    CASE ELSE: PRINT "else"
END SELECT
x = 5
SELECT CASE x
    CASE ASC(""): PRINT "case ASC(empty) for 5"
    CASE 0: PRINT "case 0 for 5"
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
