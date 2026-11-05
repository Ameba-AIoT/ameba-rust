# can/external

## Build

```bash
cargo xtask build example --name can/external
```

Default bit rate is 1 Mbps. Select another rate at build time with `CAN_BIT_RATE` (decimal bps):

```bash
CAN_BIT_RATE=500000 cargo xtask build example --name can/external
```

## Wiring

CAN0 and an external CAN node (e.g. a USB-CAN adapter) on one bus, 120 Ω termination at both ends. Configure the external node for classic CAN, normal mode (not listen-only), same bit rate as the firmware.

|     | CAN0 |
|-----|------|
| TX  | PA25 |
| RX  | PA26 |
| STB | PA3  |

STB is pulled low to put the transceiver in normal mode (correct for TJA1042-style parts; check your transceiver's datasheet). Classic CAN 2.0 only — the controller has no CAN FD support.

## Behavior

- CAN0 sends standard data frame `0x321`, DLC 8, every 500 ms; `data[0]` is a sequence number, the rest `11 22 33 44 55 66 77`. Automatic retransmission is off, so a frame counts as `tx_ok` only if another node ACKed it.
- Every frame CAN0 receives is printed and echoed back with ID + 1, same type (standard/extended, data/remote), DLC and data.
- A `stats` line is printed every 2 s.
- If the controller goes bus-off (e.g. the external node runs at a different bit rate), it rejoins the bus and prints `[CAN] bus-off, recovery N: bus on`.

To check the link from the external node:

1. It receives `0x321` every ~500 ms with an incrementing first byte, while `tx_ok` keeps rising and `tx_fail`/TEC stay at 0.
2. Send it a standard data frame, an extended data frame and a remote frame: each appears as an `rx` line with the same ID, type, DLC and data, and the node receives the echo with ID + 1.

## Expected Output

```
[CAN external] start: CAN0 @ 1000000 bps, classic CAN
[CAN] bus on, periodic id=0x321 every 500 ms, echo id+1
[CAN] stats: tx_ok=4 tx_fail=0 rx=0 echo_fail=0 TEC=0 REC=0 err=0x00000000
[CAN] rx std data id=0x100 dlc=8 data=[00, 11, 22, 33, 44, 55, 66, 77]
```

If nothing ACKs (no other node, wrong bit rate, bus not terminated), `tx_ok` stays 0, `tx_fail` rises by one per period and TEC climbs to error passive (128).

Log lines from the NP core (`[APP-I]`, `[MAIN-I]`, ...) may be interleaved.
