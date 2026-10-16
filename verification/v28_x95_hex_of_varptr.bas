$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): one form, on the last line, as code that is not reached.
DIM i AS INTEGER, l AS LONG
DIM a(1 TO 10) AS INTEGER
PRINT "ok"
SYSTEM
PRINT HEX$(VARPTR(a(9))); OCT$(VARSEG(a(1)))
