# can/loopback

## Build

```bash
cargo xtask build example --name can/loopback
```

## Wiring

No external wiring needed -- CAN0 runs in internal loopback mode (`WorkMode::InternalLoopback`): TX is tied back to RX inside the controller and the TX pin stays recessive, so no transceiver or bus is required.

This only exercises the controller (registers, message buffers, frame encoding). Pinmux, transceiver, ACK and bit timing against another node are not covered.

## Behavior

Selects XTAL as the CAN core clock, computes 1 Mbps bit timing with `calc_bit_timing`, and sends 4 frames from message buffer 0 (std data DLC 8, ext data DLC 5, std data DLC 0, std remote DLC 4). Message buffer 15 accepts every frame (all masks don't care); each received frame is checked for ID type, frame type, ID, DLC and data. PASS requires all 4 frames to match and TEC/REC to be 0.

## Expected Output

```
[CAN loopback] start
[CAN] 1000000 bps: brp=2 prop=7 ph1=7 ph2=5 sjw=2
[CAN] bus on
[CAN] case 0: std data id=0x123 dlc=8 -> rx std data id=0x123 dlc=8 data=[01, 02, 03, 04, 05, 06, 07, 08] OK
[CAN] case 1: ext data id=0x1ABCDE01 dlc=5 -> rx ext data id=0x1ABCDE01 dlc=5 data=[A5, 5A, FF, 00, 3C] OK
[CAN] case 2: std data id=0x7FF dlc=0 -> rx std data id=0x7FF dlc=0 data=[] OK
[CAN] case 3: std rtr id=0x456 dlc=4 -> rx std rtr id=0x456 dlc=4 data=[00, 00, 00, 00] OK
[PASS] CAN loopback: 4 frames OK, TEC=0 REC=0
```

Log lines from the NP core (`[APP-I]`, `[MAIN-I]`, ...) may be interleaved.
