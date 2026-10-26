DECLARE SUB topbar ()
$console:only
' Slice (m3-get-put-bload, task 2.2): a line that is the name of a SUB and a colon. With no jump to the name
' anywhere in the program it is a call of the SUB; with a jump in any body it is a label in every body, and CALL
' still calls the SUB; a SUB with a parameter and a FUNCTION are always labels (verification\v28_c_labels,
' v28_x71-x92). Civil War Strategy has the call (topbar:) and the label beside a CALL (ships:, cannon:).
dim shared n as integer
dim v as integer
call topbar
twice:
twice:
after: print "after the call"
param:
fname:
print "main"; fname
call param(3)
gosub part
call navy
onesub:
print "main goes on"
call other
thenonly:
if n > 0 then thenonly
' Each kind of jump makes its name a label.
gosub viagosub
restore viarestore
read v
print "read"; v
on error goto viaerror
error 5
print "back"
on 2 GOTO part, viaon
print "not printed"
viaon:
print "via on"
system
part:
print "part"
return
topbar:
print "not reached"
viagosub:
print "label viagosub"
return
data 1
viarestore:
data 2
viaerror:
print "label viaerror"
resume next

sub topbar
    print "sub topbar"
end sub

sub twice
    print "sub twice"
end sub

sub after
    print "sub after"
end sub

sub param (k as integer)
    print "sub param"; k
end sub

function fname
    fname = 7
end function

sub navy
    ships:
    n = n + 1
    print "navy"; n
    if n < 2 goto ships
    call ships
    cannon:
    n = n + 1
    if n < 5 then goto cannon
    call cannon
    print "navy ends"; n
end sub

sub ships
    print "sub ships"
end sub

sub cannon
    print "sub cannon"
end sub

sub onesub
    print "sub onesub"
end sub

sub other
    dim k as integer
    onesub:
    k = k + 1
    if k < 2 then goto onesub
    print "other"; k
end sub

sub thenonly
    print "sub thenonly"
end sub

sub viagosub
    print "sub viagosub"
end sub

sub viarestore
    print "sub viarestore"
end sub

sub viaerror
    print "sub viaerror"
end sub

sub viaon
    print "sub viaon"
end sub
