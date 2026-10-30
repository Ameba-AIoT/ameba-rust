# shell

## Build

```bash
cargo xtask build example --name shell
```

## Behavior

Receives UART commands from km4ns over IPC N2A channel 4 (`LOGUART_RX`), parsing and executing them on the km4tz side.

The km4ns UART shell's input is forwarded over IPC; the shell example registers a handler on that channel, tokenizes the received string, and dispatches it to the matching command function via a lookup table.

### Built-in commands

| Command | Format | Behavior |
|------|------|------|
| `dw` | `dw <hex_addr>` | Reads the 32-bit value at the given address and prints it |
| `ew` | `ew <hex_addr> <hex_val>` | Writes a 32-bit value to the given address |

### Adding commands

Add a `Cmd { name: "xxx", func: cmd_xxx }` entry to the `COMMANDS` static table in `main.rs` to register a new command.

## Expected Output

```
shell: start
shell: ready on N2A ch4
```

Typing `dw 0x40000000` on the km4ns UART:

```
[0x40000000] = 0x12345678
```

Typing `ew 0x20010000 0xdeadbeef`:

```
[0x20010000] <- 0xdeadbeef
```

Typing an unknown command:

```
shell: unknown cmd 'foo'
```
