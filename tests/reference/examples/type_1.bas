$CONSOLE:ONLY
TYPE Army
    leader AS STRING * 10
    men AS LONG
    morale AS SINGLE
END TYPE
DIM a AS Army, corps(1 TO 2) AS Army
a.leader = "Davout": a.men = 26000: a.morale = 0.9
corps(1) = a
corps(2).leader = "Lannes": corps(2).men = 20000
PRINT corps(1).leader; corps(1).men; corps(1).morale
PRINT corps(2).leader; corps(2).men
PRINT "An Army takes"; LEN(a); "bytes"
END
