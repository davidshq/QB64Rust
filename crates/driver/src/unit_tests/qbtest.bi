' The assertion library of `qb64rust test`, its first half (spec compiler/unit-tests, "The assertion library";
' OpenSpec change basic-unit-tests, design D4). The generated entry point includes it before the test file, so its
' names are typed explicitly and start with qbtest_: the test file's DEFxxx, OPTION _EXPLICIT and OPTION BASE
' cannot change them. The SUBs are in qbtest.bm, included after the entry point's SYSTEM.
DIM SHARED qbtest_result AS STRING
qbtest_result = ENVIRON$("QB64RUST_TEST_RESULT")
