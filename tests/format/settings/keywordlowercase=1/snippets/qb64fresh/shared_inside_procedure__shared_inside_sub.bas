dim globalvar as integer
globalvar = 100

sub MySub
    shared globalvar
    globalvar = globalvar + 1
end sub
