$Console:Only
' Runtime comparison: RND(negative) reseeds and returns first value of new sequence.
Randomize 1
a = Rnd
b = Rnd
first = Rnd(-5)
second = Rnd
Print "after_seed1:"; a; b
Print "after_rnd_neg5:"; first; second
End
