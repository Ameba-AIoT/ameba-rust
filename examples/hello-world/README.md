# hello-world

## Build

```bash
cargo xtask build example --name hello-world
```

## Behavior

The simplest embassy example. Initializes the HAL (sys_timer + IPC), starts
the embassy executor, and in `hello_task` prints a Hello World line via
`DiagPrintf` every 1 second.

## Expected Output

```
Hello World
Hello World
Hello World
...
```

Prints one `Hello World` line every 1 second.
