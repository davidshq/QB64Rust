$Console:Only
' Runtime comparison: RND(0) returns last generated value (no advance).
' Same seed so sequence is deterministic per runtime; RND(0) must repeat previous.
Randomize 1
a = Rnd
b = Rnd
c = Rnd(0)
Print "rnd1:"; a
Print "rnd2:"; b
Print "rnd0:"; c
Print "rnd0_eq_rnd2:"; (c = b)
End
