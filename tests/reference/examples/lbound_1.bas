$CONSOLE:ONLY
DIM board(1 TO 8, 0 TO 7)
PRINT "Rows"; LBOUND(board); "to"; UBOUND(board)
PRINT "Columns"; LBOUND(board, 2); "to"; UBOUND(board, 2)
DIM plain(5)
PRINT "A dimension without TO starts at"; LBOUND(plain)
END
