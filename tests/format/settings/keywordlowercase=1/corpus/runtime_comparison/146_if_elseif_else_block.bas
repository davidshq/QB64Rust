$console:only
' Runtime comparison: IF / ELSEIF / ELSE blocks, PRINT per branch
dim x as long
x = 1
if x = 1 then
    print "branch one"
elseif x = 2 then
    print "branch two"
else
    print "branch else"
end if
x = 2
if x = 1 then
    print "branch one"
elseif x = 2 then
    print "branch two"
else
    print "branch else"
end if
x = 3
if x = 1 then
    print "branch one"
elseif x = 2 then
    print "branch two"
else
    print "branch else"
end if
end
