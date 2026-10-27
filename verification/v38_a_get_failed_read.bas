$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-measured-copies, before its design; QB64PE-DIVERGENCES.md D-021): what a GET of a whole array of a
' type with a _DYNAMIC member array leaves in the array when its read raises an error. The old compiler unpacks
' its buffer after the call whatever the read did (qb64pe.bas 12398); the buffer belongs to the statement
' (one per GET), so it holds what that statement read last, or nothing if it never read.
' 1. the first run of a GET whose file is not open: the buffer was never written
' 2. the same GET after it read a file, then with the file closed: the buffer holds the earlier read
' 3. a GET of a random file whose record is shorter than the array's bytes (error 59, Bad record length)
' 4. a GET past the end of a binary file (no error: what is read)
TYPE t
    a AS INTEGER
    v(1) _DYNAMIC AS LONG
END TYPE
ON ERROR GOTO h
DIM i AS LONG, pass AS LONG
REDIM p(1) AS t
REDIM q(1) AS t
FOR i = 0 TO 1
    q(i).a = 10 + i: q(i).v(0) = 100 + i: q(i).v(1) = 200 + i
NEXT
OPEN "v38_a.bin" FOR BINARY AS #1
PUT #1, 1, q()
PRINT "PUT: LOF ="; LOF(1)
CLOSE #1

PRINT "1. never read, file not open:";
GOSUB fill
GET #2, 1, p()
GOSUB show

FOR pass = 1 TO 2
    IF pass = 1 THEN OPEN "v38_a.bin" FOR BINARY AS #3
    PRINT "2."; pass; ":";
    GOSUB fill
    GET #3, 1, p()
    GOSUB show
    IF pass = 1 THEN CLOSE #3
NEXT

OPEN "v38_a.bin" FOR RANDOM AS #4 LEN = 4
PRINT "3. record too short:";
GOSUB fill
GET #4, 1, p()
GOSUB show
CLOSE #4

OPEN "v38_a.bin" FOR BINARY AS #5
PRINT "4. past the end:";
GOSUB fill
GET #5, 15, p()
GOSUB show
CLOSE #5

KILL "v38_a.bin"
PRINT "ok"
SYSTEM

fill:
FOR i = 0 TO 1
    p(i).a = -1: p(i).v(0) = -2: p(i).v(1) = -3
NEXT
RETURN

show:
FOR i = 0 TO 1
    PRINT p(i).a; p(i).v(0); p(i).v(1);
NEXT
PRINT
RETURN

h:
PRINT " [error"; ERR; "]";
RESUME NEXT
