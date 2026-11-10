' QB 4.5 (beside SOMEDAY's row on the order of a store through a member with an index on the way, v24_x316):
' when the index calls a FUNCTION that changes the value being stored, which value is stored? QB64pe reads a
' string after the index and a number before it. QB 4.5 has no STRING member, so STRING * 6 stands in.
DECLARE FUNCTION setz% (i AS INTEGER)
DECLARE FUNCTION setn% (i AS INTEGER)
TYPE t1
    f AS STRING * 6
    k AS LONG
END TYPE
DIM a(3) AS t1, b$(3), c(3) AS LONG
DIM SHARED z$, n AS LONG
OPEN "OUT.TXT" FOR OUTPUT AS #1
z$ = "before"
a(setz(1)).f = z$
PRINT #1, "1. a(setz(1)).f = z$: ["; a(1).f; "]"
z$ = "before"
a(setz(1)).f = z$ + "!"
PRINT #1, "2. a(setz(1)).f = z$ + excl: ["; a(1).f; "]"
n = 1
a(setn(1)).k = n
PRINT #1, "3. a(setn(1)).k = n:"; a(1).k
z$ = "before"
b$(setz(1)) = z$
PRINT #1, "4. b$(setz(1)) = z$: ["; b$(1); "]"
n = 1
c(setn(1)) = n
PRINT #1, "5. c(setn(1)) = n:"; c(1)
CLOSE #1
SYSTEM

FUNCTION setn% (i AS INTEGER)
    n = 2
    setn% = i
END FUNCTION

FUNCTION setz% (i AS INTEGER)
    z$ = "after"
    setz% = i
END FUNCTION
