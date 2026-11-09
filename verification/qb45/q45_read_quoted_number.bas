' QB 4.5 (beside SOMEDAY's row on READ of a quoted item into a number): DATA "5" read into a LONG, an INTEGER and
' a SINGLE, and the unquoted 5 beside it. QB64pe raises a syntax error (2) for the quoted item.
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
n& = -1: READ n&
PRINT #1, "1. DATA 5 into a LONG:"; n&
n& = -1: READ n&
PRINT #1, "2. DATA "; CHR$(34); "5"; CHR$(34); " into a LONG:"; n&
i% = -1: READ i%
PRINT #1, "3. the same into an INTEGER:"; i%
s! = -1: READ s!
PRINT #1, "4. DATA "; CHR$(34); "2.5"; CHR$(34); " into a SINGLE:"; s!
READ t$
PRINT #1, "5. the next item, as a string: ["; t$; "]"
CLOSE #1
SYSTEM
h:
PRINT #1, " error"; ERR;
RESUME NEXT
DATA 5, "5", "5", "2.5", after
