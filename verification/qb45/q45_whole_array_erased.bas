' QB 4.5 (row 7 of the crash-class rulings, study\39): the whole-array forms QB64pe takes for
' an erased array (verification\v54_d_whole_erased): READ a(), SWAP a(), b(), INPUT #1, a(); and an element
' of an erased array for comparison.
ON ERROR GOTO h
REDIM a(2) AS LONG, b(2) AS LONG
ERASE a
OPEN "OUT.TXT" FOR OUTPUT AS #1
PRINT #1, "element"; a(0)
DATA 9
READ a()
PRINT #1, "after READ"
SWAP a(), b()
PRINT #1, "after SWAP"
CLOSE #1
SYSTEM
h:
PRINT #1, "[error"; ERR; "]"
RESUME NEXT
