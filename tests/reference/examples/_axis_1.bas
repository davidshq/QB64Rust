$CONSOLE:ONLY
ON ERROR GOTO trap
PRINT "Before a device is selected:"
y# = _AXIS(1)
y# = _WHEEL(1)
x = _DEVICEINPUT(2)
PRINT "After selecting the mouse:"
y# = _AXIS(99)
y# = _WHEEL(99)
END
trap:
PRINT " error"; ERR
RESUME NEXT
