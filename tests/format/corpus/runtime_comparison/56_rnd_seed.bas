$Console:Only
' Runtime comparison: RANDOMIZE, RND (output may differ between runtimes)
Randomize 1
Print "rnd1:"; Rnd
Print "rnd2:"; Rnd
Print "rnd0:"; Rnd(0)
End
