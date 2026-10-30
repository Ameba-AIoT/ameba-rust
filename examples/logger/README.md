# logger

## Build

```bash
cargo xtask build example --name logger
```

## Wiring

No external wiring needed.

## Behavior

Calls `ameba_hal::logger::init(LevelFilter::Info)` to wire the standard `log` crate into LOGUART (internally reuses `println!`/`LogWriter`, synchronous blocking output). Prints all five levels in turn -- `error!`/`warn!`/`info!`/`debug!`/`trace!` -- of which `debug!`/`trace!` should be filtered out by `LevelFilter::Info`, so the serial log should show only the first three.

## Expected Output

```
[LOGGER] start
[ERROR] logger: error level
[WARN] logger: warn level
[INFO] logger: info level
[PASS] logger: OK
```

The `debug!`/`trace!` lines **should not** appear -- if they do, `LevelFilter` filtering isn't working.
