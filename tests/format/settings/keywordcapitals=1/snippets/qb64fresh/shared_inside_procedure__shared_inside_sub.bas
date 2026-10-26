DIM globalvar AS INTEGER
globalvar = 100

SUB MySub
    SHARED globalvar
    globalvar = globalvar + 1
END SUB
