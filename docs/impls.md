# Implementations

A working document to track milestones and improvements.

## Naive implementation

The initial implementation opens the `File` and wraps it in a `BufReader`.

Each line (`\n`) is read and parsed from the buffered reader into a `BTreeMap`. The `BTreeMap` means that the keys are automatically stored in sorted order for iterating over later.

The min/mean/max values are calculated from a running total within the map. As station names are "unique", this means that duplicate entries can modify the currently stored value or create one if it does not yet exist.

# Background workers

This shifts the processing step into the background. Instead of happening on the hot path for reads and calculating the values on the fly, a "chunk" is accumulated in memory and handed off to a background thread for processing. The result many workers are merged together through their respective `JoinHandle`s into the `BTreeMap`.
