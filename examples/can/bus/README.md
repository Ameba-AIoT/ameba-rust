# can/bus

## Build

```bash
cargo xtask build example --name can/bus
```

## Wiring

CAN0 and CAN1 on one real bus. Each controller drives its own transceiver; connect the two CAN_H/CAN_L pairs (straight, not crossed) and make sure the bus has 120 Ω termination at both ends.

|     | CAN0 | CAN1 |
|-----|------|------|
| TX  | PA25 | PB17 |
| RX  | PA26 | PB18 |
| STB | PA3  | PB19 |

STB is pulled low to put the transceivers in normal mode (correct for TJA1042-style parts; check your transceiver's datasheet). Classic CAN 2.0 only — the controller has no CAN FD support.

## Behavior

Brings both controllers onto the bus at 1 Mbps (bit timing from `calc_bit_timing`) and sends 4 frames in each direction (std data DLC 8, ext data DLC 5, std data DLC 0, std remote DLC 4). Every received frame is checked for ID type, frame type, ID, DLC and data; TEC/REC must stay 0.

## Expected Output

```
[CAN bus] start
[CAN] 1000000 bps: bus on
[CAN] CAN0->CAN1 4/4, CAN1->CAN0 4/4, TEC0=0 REC0=0 TEC1=0 REC1=0
[PASS] CAN bus: 8 frames OK, TEC=REC=0
```

Log lines from the NP core (`[APP-I]`, `[MAIN-I]`, ...) may be interleaved.
