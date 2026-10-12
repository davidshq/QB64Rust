$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 8): a user-type parameter given a nested member of an element; a
' whole value copied into and out of a parameter; a parameter's STRING member passed on by reference; a STRING
' member as a target of READ, of the MID$ statement and of SWAP, and named with its suffix.
TYPE pt
    x AS LONG
    n AS STRING
END TYPE
TYPE box
    k AS LONG
    a AS pt
END TYPE
DIM q AS pt, r AS pt, bs(2) AS box
bs(1).a.x = 4: bs(1).a.n = "el"
move bs(1).a
PRINT "1. a nested member of an element:"; bs(1).a.x; bs(1).a.n
q.x = 1: q.n = "q"
copyin q, bs(1).a
PRINT "2. copied in the SUB, p = s:"; q.x; q.n
r.x = 9: r.n = "r"
copyout r
PRINT "3. a local copied to the parameter:"; r.x; r.n
shout q
PRINT "4. the parameter's STRING member passed on: "; q.n
READ q.n, q.x
PRINT "5. READ: "; q.n; q.x
MID$(q.n, 2, 1) = "#"
PRINT "6. MID$: "; q.n
r.n = "other"
SWAP q.n, r.n
PRINT "7. SWAP of two STRING members: "; q.n; " "; r.n
q.n$ = "suffix"
PRINT "8. named with its suffix: "; q.n$; LEN(q.n$)
IF q.n = "suffix" AND q.n > r.n THEN PRINT "9. compared"
SYSTEM
DATA data,12

SUB move (p AS pt)
    p.x = p.x + 1
    p.n = p.n + "m"
END SUB
SUB copyin (p AS pt, s AS pt)
    p = s
END SUB
SUB copyout (p AS pt)
    DIM l AS pt
    l.x = 3: l.n = "local"
    p = l
END SUB
SUB shout (p AS pt)
    bang p.n
END SUB
SUB bang (s AS STRING)
    s = s + "!"
END SUB
