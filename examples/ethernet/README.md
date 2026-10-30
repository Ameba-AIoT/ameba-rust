# ethernet

## Build

```bash
cargo xtask build example --name ethernet
```

## Wiring

No external PHY needed -- the example uses a no-op `NoPhy` stub to verify the MAC controller's init flow. With a real PHY connected you'll get `[ETH] MAC init OK`; without one you'll get a timeout message.

## Behavior

Initializes the Ethernet MAC (100M full-duplex, 8 TX / 10 RX descriptors -- matching the SDK's `ETH_StructInit()` defaults, 1536-byte buffers) and verifies the init path doesn't panic.

## Expected Output

No PHY (the normal case):

```
[ETH] start
[ETH] timeout (no PHY) -- compile OK
[PASS] ethernet: init OK
```

With a PHY connected:

```
[ETH] start
[ETH] MAC init OK
[PASS] ethernet: init OK
```
