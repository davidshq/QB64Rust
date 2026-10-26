DECLARE SUB topbar ()
$CONSOLE:ONLY
' Slice (m3-get-put-bload, task 2.2): a line that is the name of a SUB and a colon. With no jump to the name
' anywhere in the program it is a call of the SUB; with a jump in any body it is a label in every body, and CALL
' still calls the SUB; a SUB with a parameter and a FUNCTION are always labels (verification\v28_c_labels,
' v28_x71-x92). Civil War Strategy has the call (topbar:) and the label beside a CALL (ships:, cannon:).
DIM SHARED n AS INTEGER
DIM v AS INTEGER
CALL topbar
twice:
twice:
after: PRINT "after the call"
param:
fname:
PRINT "main"; fname
CALL param(3)
GOSUB part
CALL navy
onesub:
PRINT "main goes on"
CALL other
thenonly:
IF n > 0 THEN thenonly
' Each kind of jump makes its name a label.
GOSUB viagosub
RESTORE viarestore
READ v
PRINT "read"; v
ON ERROR GOTO viaerror
ERROR 5
PRINT "back"
ON 2 GOTO part, viaon
PRINT "not printed"
viaon:
PRINT "via on"
SYSTEM
part:
PRINT "part"
RETURN
topbar:
PRINT "not reached"
viagosub:
PRINT "label viagosub"
RETURN
DATA 1
viarestore:
DATA 2
viaerror:
PRINT "label viaerror"
RESUME NEXT

SUB topbar
    PRINT "sub topbar"
END SUB

SUB twice
    PRINT "sub twice"
END SUB

SUB after
    PRINT "sub after"
END SUB

SUB param (k AS INTEGER)
    PRINT "sub param"; k
END SUB

FUNCTION fname
    fname = 7
END FUNCTION

SUB navy
    ships:
    n = n + 1
    PRINT "navy"; n
    IF n < 2 GOTO ships
    CALL ships
    cannon:
    n = n + 1
    IF n < 5 THEN GOTO cannon
    CALL cannon
    PRINT "navy ends"; n
END SUB

SUB ships
    PRINT "sub ships"
END SUB

SUB cannon
    PRINT "sub cannon"
END SUB

SUB onesub
    PRINT "sub onesub"
END SUB

SUB other
    DIM k AS INTEGER
    onesub:
    k = k + 1
    IF k < 2 THEN GOTO onesub
    PRINT "other"; k
END SUB

SUB thenonly
    PRINT "sub thenonly"
END SUB

SUB viagosub
    PRINT "sub viagosub"
END SUB

SUB viarestore
    PRINT "sub viarestore"
END SUB

SUB viaerror
    PRINT "sub viaerror"
END SUB

SUB viaon
    PRINT "sub viaon"
END SUB
