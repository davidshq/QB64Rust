$console:only
' Runtime comparison: Deeply nested IF/FOR/SELECT
dim i as long, j as long, k as long
for i = 1 to 2
    for j = 1 to 2
        for k = 1 to 2
            if i = 1 then
                if j = 1 then
                    if k = 1 then
                        print "111"
                    else
                        print "112"
                    end if
                else
                    print "12x"
                end if
            else
                print "2xx"
            end if
        next k
    next j
next i
print "done"
end
