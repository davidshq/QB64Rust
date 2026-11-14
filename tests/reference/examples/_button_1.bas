$CONSOLE:ONLY
ON ERROR GOTO trap
PRINT "Before a device is selected:"
x = _BUTTON(1)
x = _BUTTONCHANGE(1)
x = _DEVICEINPUT(2)
PRINT "After selecting the mouse:"
x = _BUTTON(0)
x = _BUTTONCHANGE(99)
END
trap:
PRINT " error"; ERR
RESUME NEXT
