$CONSOLE:ONLY
' Verification (m3-events, task 1.1): KEY in a console program. KEY ON, KEY OFF, KEY LIST, KEY n, text$ and a
' user key; the switches KEY(k) ON|OFF|STOP and KEY(0); the run-time errors of ON KEY and KEY(k) by number,
' the number as a float (rounded or cut?), and of KEY n, text$.
ON ERROR GOTO oops
PRINT "1. KEY ON:";: KEY ON: PRINT " done"
PRINT "2. KEY OFF:";: KEY OFF: PRINT " done"
PRINT "3. KEY 1, abc:";: KEY 1, "abc": PRINT " done"
PRINT "4. KEY LIST:"
KEY LIST
PRINT "5. KEY 15, a user key:";: KEY 15, CHR$(0) + CHR$(30): PRINT " done"
PRINT "6. ON KEY(15):";: ON KEY(15) GOSUB h: PRINT " done"
PRINT "7. KEY(0) ON:";: KEY(0) ON: PRINT " done"
PRINT "8. KEY(1) STOP:";: KEY(1) STOP: PRINT " done"
PRINT "9. KEY(31) OFF:";: KEY(31) OFF: PRINT " done"
PRINT "10. ON KEY(0):";: ON KEY(0) GOSUB h: PRINT " done"
PRINT "11. ON KEY(32):";: ON KEY(32) GOSUB h: PRINT " done"
PRINT "12. ON KEY(0.7):";: ON KEY(0.7) GOSUB h: PRINT " done"
PRINT "13. ON KEY(31.6):";: ON KEY(31.6) GOSUB h: PRINT " done"
PRINT "14. ON KEY(-0.7):";: ON KEY(-0.7) GOSUB h: PRINT " done"
PRINT "15. KEY(32) ON:";: KEY(32) ON: PRINT " done"
PRINT "16. KEY(-1) ON:";: KEY(-1) ON: PRINT " done"
PRINT "17. KEY(31.6) ON:";: KEY(31.6) ON: PRINT " done"
PRINT "18. KEY 0, x:";: KEY 0, "x": PRINT " done"
PRINT "19. KEY 40, x:";: KEY 40, "x": PRINT " done"
PRINT "20. KEY 1.6, x:";: KEY 1.6, "x": PRINT " done"
PRINT "21. KEY 12, x:";: KEY 12, "x": PRINT " done"
PRINT "22. KEY 15, too long:";: KEY 15, "abc": PRINT " done"
PRINT "23. KEY 2, empty:";: KEY 2, "": PRINT " done"
PRINT "24. KEY 2, 20 characters:";: KEY 2, STRING$(20, "z"): PRINT " done"
KEY LIST
SYSTEM

h:
PRINT "key"
RETURN

oops:
PRINT " error"; ERR;
RESUME NEXT
