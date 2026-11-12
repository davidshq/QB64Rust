# STRIG

`STRIG(n)` tells whether a game controller's button is down, or was pressed since the last call. As a statement,
`STRIG(n) ON`, `OFF` and `STOP` switch the event that `ON STRIG` handles.

## Syntax

<!-- begin generated: syntax -->
```text
STRIG(button%) {On|Off|Stop}
STRIG(button_function%) or STRIG(button_function%, device_number%)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

```text
ON STRIG(n[, controller]) GOSUB label
ON STRIG(n[, controller]) subname [argument]
```

## Arguments

- `n`: the button and the test. `n \ 4 + 1` is the button. An even `n` asks whether it was pressed since the last
  call, an odd `n` whether it is down now. With no `controller`, `n AND 2` picks the second controller: 0 and 1 are
  button 1 of the first, 2 and 3 button 1 of the second, 4 and 5 button 2 of the first.
- `controller`: the controller, from 1 to 65535.

## Result

-1 or 0, a LONG.

## Behavior

With no game controller attached, `STRIG(0)`, `STRIG(1)` and `STRIG(4, 1)` are 0.
<!-- src: language/device-input "STICK and STRIG"; verification\v47_b_devices; tests\reference\examples\strig_1 -->

`ON STRIG(n) GOSUB label` sets the handler of a button, a label of the main module or a SUB with an optional integer
argument, as `ON KEY` does. `STRIG(n) ON`, `OFF` and `STOP` switch it; `STRIG ON` with no number switches every
button.
<!-- src: language/events "Joystick-button handlers"; verification\v42_x11_strig; tests\reference\examples\strig_1 -->

A program that uses it is built with libqb's device-input part, as QB64pe builds it.
<!-- src: language/device-input "The device-input runtime part" -->

## Errors

- `n` outside 0 to 65535, or `controller` outside 1 to 65535, raises error 5: `STRIG(70000)` does.
  <!-- src: language/device-input "STICK and STRIG"; verification\v47_b_devices; tests\reference\examples\strig_1 -->
- `ON STRIG` with an odd `n` raises error 5 when it runs, since "down now" is no event.
  <!-- src: language/events "Joystick-button handlers"; read: libqb\internal\c\qbx.cpp:1137-1172; tests\corpus\slice\s149_devices_console -->
- A string argument is a compile error, as is `STRIG(1) = 2` or `STRIG(1, 2, 3) ON`.
  <!-- src: language/device-input "Errors of the device functions"; verification\v47_x22; verification\v55_x04_strig_place; verification\v47_x19 -->
- A wrong `ON STRIG` form is a compile error: no number, three numbers, no `GOSUB` or SUB name, or a SUB whose
  argument is not an integer type.
  <!-- src: language/events "Joystick-button handlers"; verification\v47_x04; verification\v47_x09; verification\v47_x12 -->

## Availability

<!-- begin generated: availability -->
Supported. All 18 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:strig_1 -->
```qb64
ON ERROR GOTO trap
PRINT "With no game controller:"; STRIG(0); STRIG(1); STRIG(4, 1)
ON STRIG(0) GOSUB pressed
STRIG(0) ON
STRIG OFF
x = STRIG(70000)
PRINT "done"
END
pressed:
PRINT "pressed"
RETURN
trap:
PRINT "STRIG(70000) raised error"; ERR
RESUME NEXT
```

Output:

```text
With no game controller: 0  0  0 
STRIG(70000) raised error 5 
done

Press any key to continue
```
<!-- end generated -->

## See also

`STICK`, `ON`, `_BUTTON`.
