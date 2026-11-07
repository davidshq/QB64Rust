' QB 4.5, PDS 7.1 and QB64: what INPUT # reads into an INTEGER from the text WRITE # gives for
' .01 * s&, a SINGLE constant times a LONG. Found by the Wars of Napoleon port
' (Hutsell-Games/wars-of-napoleon, docs\devnotes.md, step 2 section 4): the tactical game writes
' each side's losses as .01 * score& into OUTCOME.&&&, and the strategic game reads them into
' INTEGERs. The old compilers write a DOUBLE (1.4999999665... for 150), QB64 a SINGLE (1.5).
' Part 1: INPUT # of fixed texts into an INTEGER, to show how it rounds a half.
' Part 2: the text WRITE # gives for .01 * s& at a few values, and what INPUT # reads back.
' Part 3: for s& = 0 to 3000, every value where the INTEGER read back is not the nearest
' integer to s& / 100 rounded down at a half (the old compilers' result, since their product is
' just under each half).
' Part 4: three forms meant to make QB64 multiply the SINGLE constant by the LONG in DOUBLE,
' as the old compilers do: .01! * CDBL(s&), k! * CDBL(s&) with k! = .01, CSNG(.01) * CDBL(s&).
DEFINT A-Z
OPEN "OUT.TXT" FOR OUTPUT AS #1
PRINT #1, "part 1"
OPEN "T.TXT" FOR OUTPUT AS #2
PRINT #2, "0.5,1.5,2.5,3.5,4.5,-0.5,-1.5,-2.5,1.4999999,1.5000001,2.4999999,2.5000001,.69,6.5,7.5"
CLOSE #2
OPEN "T.TXT" FOR INPUT AS #2
l$ = ""
FOR k = 1 TO 15
INPUT #2, i
l$ = l$ + STR$(i)
NEXT k
CLOSE #2
PRINT #1, l$
PRINT #1, "part 2"
DIM s&(10)
s&(1) = 50: s&(2) = 69: s&(3) = 150: s&(4) = 250: s&(5) = 350: s&(6) = 450
s&(7) = 550: s&(8) = 1250: s&(9) = 2450: s&(10) = 149
FOR k = 1 TO 10
OPEN "T.TXT" FOR OUTPUT AS #2
WRITE #2, 1, .01 * s&(k), .01 * s&(k)
CLOSE #2
OPEN "T.TXT" FOR INPUT AS #2
LINE INPUT #2, t$
CLOSE #2
OPEN "T.TXT" FOR INPUT AS #2
INPUT #2, w, a, b
CLOSE #2
PRINT #1, s&(k); t$; a; b
NEXT k
PRINT #1, "part 3"
n = 0: l$ = ""
FOR v& = 0 TO 3000
OPEN "T.TXT" FOR OUTPUT AS #2
WRITE #2, .01 * v&
CLOSE #2
OPEN "T.TXT" FOR INPUT AS #2
INPUT #2, a
CLOSE #2
e = v& \ 100
IF v& MOD 100 > 50 THEN e = e + 1
IF a <> e THEN
n = n + 1
l$ = l$ + STR$(v&) + ":" + LTRIM$(STR$(a))
IF LEN(l$) > 70 THEN PRINT #1, l$: l$ = ""
END IF
NEXT v&
IF l$ <> "" THEN PRINT #1, l$
PRINT #1, "differ"; n
k! = .01
FOR f = 1 TO 3
PRINT #1, "part 4, form"; f
n = 0: l$ = ""
FOR v& = 50 TO 2950 STEP 100
OPEN "T.TXT" FOR OUTPUT AS #2
SELECT CASE f
CASE 1: WRITE #2, .01! * CDBL(v&)
CASE 2: WRITE #2, k! * CDBL(v&)
CASE 3: WRITE #2, CSNG(.01) * CDBL(v&)
END SELECT
CLOSE #2
OPEN "T.TXT" FOR INPUT AS #2
LINE INPUT #2, t$
CLOSE #2
OPEN "T.TXT" FOR INPUT AS #2
INPUT #2, a
CLOSE #2
IF v& = 150 THEN PRINT #1, t$
IF a <> v& \ 100 THEN n = n + 1
NEXT v&
PRINT #1, "differ"; n
NEXT f
CLOSE #1
SYSTEM
