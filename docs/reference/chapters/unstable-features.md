# Unstable features

`$UNSTABLE` turns on a feature QB64pe has not made a stable part of the language.

The pages of this area: [$UNSTABLE](../entries/meta-unstable.md).

## The flags

`$UNSTABLE:TYPEFIELDS` allows the `_STATIC` and `_DYNAMIC` markers of member arrays in the `TYPE` blocks below it.
<!-- src: language/unstable-features "$UNSTABLE flags" -->

`$UNSTABLE:MIDI` and `$UNSTABLE:HTTP` are accepted with the warning "No longer required, feature is stable now"; any
other flag is a compile error.
<!-- src: language/unstable-features "$UNSTABLE flags" -->
