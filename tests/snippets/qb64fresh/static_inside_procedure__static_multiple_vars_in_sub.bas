SUB TrackCalls
    STATIC callCount AS LONG
    STATIC lastValue AS DOUBLE
    callCount = callCount + 1
    lastValue = callCount * 1.5
END SUB
