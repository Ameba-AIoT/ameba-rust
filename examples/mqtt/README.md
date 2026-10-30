# mqtt

## Build

```bash
cargo xtask build example --name mqtt
```

Set `WIFI_SSID` and `WIFI_PASSWORD` at the top of `src/main.rs` before building.

## Wiring

No external wiring needed.

## Network Setup

The example needs a 2.4 GHz WPA2-PSK network and an MQTT broker reachable on
plain TCP port 1883, with anonymous access allowed. The example speaks MQTT v5
and has been tested with Mosquitto 2.1, for which a minimal configuration is:

```
listener 1883 0.0.0.0
allow_anonymous true
```

By default the broker address is taken from the DHCP gateway, which suits the
common setup of a phone hotspot that also runs the broker. To use a broker
elsewhere on the network, set `BROKER_IP` in `src/main.rs`, e.g.
`Some([192, 168, 1, 10])`. The address must be an IPv4 literal: this example
has no DNS resolver.

## Behavior

Joins the WiFi network, obtains an address over DHCP, opens a TCP connection to
the broker with `embassy-net`, and runs an MQTT v5 session with the
[`rust-mqtt`](https://crates.io/crates/rust-mqtt) client (QoS 0, keep-alive 20 s,
client ID `ameba-` followed by the last three MAC bytes in hex, so several
boards can share one broker):

1. Subscribes to `ameba/cmd`.
2. Publishes `hello from ameba` to `ameba/cmd` and waits up to 5 s for it to
   come back through the broker, printing `[PASS] mqtt: loopback`. Any other
   message that arrives on `ameba/cmd` in the meantime is printed as
   `ignored during self-check` and does not fail the check.
3. Every 2 s, publishes `count=N` to `ameba/status` and prints any message
   that has arrived on `ameba/cmd`.

From another MQTT client on the same network, you can watch the board and send
it commands:

```bash
mosquitto_sub -h <broker-ip> -t 'ameba/#' -v
mosquitto_pub -h <broker-ip> -t ameba/cmd -m hi
```

Incoming messages are checked once per 2 s cycle, so a command can take up to
2 s to appear. The example does not reconnect: if the WiFi link, the TCP
connection or the MQTT session drops, it prints a `[FAIL]` line and stops;
reset the board to start over. TLS is not supported.

## Expected Output

```
[build] <build timestamp>
[wifi] on: 0
[wifi] connecting to "<ssid>" ...
[wifi] connect returned: 0
[wifi] join_status=6
[NET] WiFi connected
[NET] DHCP: IP=192.168.x.x
[MQTT] connecting to 192.168.x.x:1883 ...
[MQTT] connected
[MQTT] subscribed: ameba/cmd
[MQTT] published: ameba/cmd = "hello from ameba"
[MQTT] received: ameba/cmd = "hello from ameba"
[PASS] mqtt: loopback
[MQTT] published: ameba/status = "count=0"
[MQTT] published: ameba/status = "count=1"
...
```

Earlier `[wifi] join_status=...` lines appear while the join progresses, and a
`[NET] WiFi (re)connected` line may appear when the link finishes coming up.

After `mosquitto_pub -t ameba/cmd -m hi`:

```
[MQTT] received: ameba/cmd = "hi"
```

If the broker cannot be reached (wrong address, broker not running, or a
firewall blocking inbound port 1883 on the broker host):

```
[FAIL] mqtt: TCP connect timed out after 10s
```

If the broker accepts the connection but the published message never comes
back (for example, the broker does not deliver a client's own publishes):

```
[FAIL] mqtt: loopback timed out after 5s
```
