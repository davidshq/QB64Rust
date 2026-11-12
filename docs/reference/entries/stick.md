# STICK

`STICK(n)` gives the position of one axis of a game controller, from 1 to 254 with 127 at the center. With no
controller attached it gives 128.

## Syntax

<!-- begin generated: syntax -->
```text
STICK(direction%) or STICK(direction%, axis_number%)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `n`: the axis. 0 and 1 are the first controller's x and y, 2 and 3 the second's, and so on.
- `pair`: which pair of the controller's axes, from 1 (the default) to 65535.

## Result

A LONG.

## Behavior

With no game controller attached, `STICK(0)`, `STICK(1)` and `STICK(2, 1)` are 128.
<!-- src: language/device-input "STICK and STRIG"; verification\v47_b_devices; tests\reference\examples\stick_1 -->

Otherwise it scales the axis's last value, from -1 to 1, to 1 to 254. A value within 0.01 of 0 gives 127.
<!-- src: read: libqb\internal\c\libqb.cpp:23988-24033 -->

A program that uses it is built with libqb's device-input part, as QB64pe builds it.
<!-- src: language/device-input "The device-input runtime part" -->

`STICK` may still name a member of a `TYPE`: `o.stick = 4` compiles and runs.
<!-- src: verification\v61_m_stick -->

## Errors

- `n` outside 0 to 65535, or `pair` outside 1 to 65535, raises error 5: `STICK(0, 0)` does.
  <!-- src: language/device-input "STICK and STRIG"; verification\v47_b_devices; tests\reference\examples\stick_1 -->
- `STICK` with no argument or with three is a compile error, "Incorrect number of arguments".
  <!-- src: language/device-input "Errors of the device functions"; verification\v47_x15; verification\v47_x23 -->

## Availability

<!-- begin generated: availability -->
Supported. All 11 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:stick_1 -->
```qb64
ON ERROR GOTO trap
PRINT "With no game controller:"; STICK(0); STICK(1); STICK(2, 1)
x = STICK(0, 0)
PRINT "done"
END
trap:
PRINT "STICK(0, 0) raised error"; ERR
RESUME NEXT
```

Output:

```text
With no game controller: 128  128  128 
STICK(0, 0) raised error 5 
done

Press any key to continue
```
<!-- end generated -->

## See also

`STRIG`, `_AXIS`, `_DEVICES`.
