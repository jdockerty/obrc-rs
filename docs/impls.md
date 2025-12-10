# Implementations

A working document to track milestones and improvements.

## Naive implementation

Commit: `260e86af35bda11a776169a2e89879ea1a2c400a`

The initial implementation opens the `File` and wraps it in a `BufReader`.

Each line (`\n`) is read and parsed from the buffered reader into a `BTreeMap`. The `BTreeMap` means that the keys are automatically stored in sorted order for iterating over later.

The min/mean/max values are calculated from a running total within the map. As station names are "unique", this means that duplicate entries can modify the currently stored value or create one if it does not yet exist.
