# ota-http

## Build

```bash
cargo xtask build example --name ota-http
```

Before building, set these constants at the top of `src/main.rs`:

| Constant | Meaning |
|:---|:---|
| `WIFI_SSID`, `WIFI_PASSWORD` | 2.4 GHz WPA2-PSK network to join |
| `OTA_SERVER_IP` | HTTP server address; must be an IPv4 literal (no DNS) |
| `OTA_SERVER_PORT` | HTTP server port |
| `OTA_PATH` | Path of the OTA image on that server |

## Wiring

No external wiring needed.

## Behavior

Downloads an OTA image over plain HTTP and programs it into the inactive slot of
the A/B flash layout:

1. Enables the system watchdog (5 s timeout) and starts a task that refreshes
   it every 500 ms. Once enabled, the watchdog stays on across resets, so every
   image that may run on the board afterwards must keep refreshing it.
2. Joins the WiFi network (40 s limit) and waits up to 45 s for DHCP.
3. Sends `GET http://<OTA_SERVER_IP>:<OTA_SERVER_PORT><OTA_PATH>` and writes
   the response body to the inactive slot.
4. On success, commits the new slot and resets the board, so the bootloader
   boots the new image. On any failure, the slot is never committed: the
   running image stays selected and the board idles without resetting.

`REQUEST_OTA_UPDATE` decides whether a boot attempts an update at all, and
`BUILD_MARKER` is printed at startup so you can tell which image is running.
Both are compile-time constants.

## Running an Update

The image to download is the `ota_all.bin` that `cargo xtask build` produces
next to `app.bin`. Each image's version is set by `IMG_VER_MAJOR` and
`IMG_VER_MINOR` in `ameba-flash/src/pack.rs`. At every boot the bootloader
compares the versions of the two slots and boots the higher one (on a tie it
picks the first slot), so the downloaded image needs a higher version than the
running one; otherwise the update succeeds but the board keeps booting the old
image.

1. Set the WiFi and server constants (see Build), keep `REQUEST_OTA_UPDATE = true`,
   then build and flash this image. It is the one that performs the update.
2. Build the image to download: set `REQUEST_OTA_UPDATE = false` so it does not
   start another update on its first boot, change `BUILD_MARKER`, and raise
   `IMG_VER_MINOR`. Copy the resulting `build/rtl8721f/ota_all.bin` to a
   directory on the server host. Do not flash this build.
3. Serve the image over HTTP with any static file server, so that it is
   reachable at `OTA_PATH` on `OTA_SERVER_PORT` of `OTA_SERVER_IP` (by default
   `http://192.168.1.100:8080/files/ota_all.bin`). If your server uses a
   different port or path, change those constants in the image from step 1.
4. Reset the board. After the download it resets on its own and prints the new
   `BUILD_MARKER`.

## Expected Output

Running image, with an update (`[wifi] join_status=...` lines omitted):

```
[build] <build timestamp>
[marker] A (currently running)
[wdg] enabled, timeout=5000ms
[wifi] on: 0
[wifi] connecting to "<ssid>" ...
[wifi] connect returned: 0
[NET] WiFi connected
[NET] MAC: xx:xx:xx:xx:xx:xx
[NET] waiting for DHCP ...
[NET] DHCP: IP=192.168.x.x
[ota] GET http://192.168.x.y:8080/files/ota_all.bin
[ota] programmed: fw_ver=... img_id=... body_len=... bytes
[PASS] ota-http: image committed to inactive slot, resetting
```

Downloaded image, after the reset:

```
[build] <build timestamp>
[marker] <your new BUILD_MARKER>
[wdg] enabled, timeout=5000ms
[ota] REQUEST_OTA_UPDATE=false, skipping OTA this boot
```

If the server cannot be reached:

```
[FAIL] ota-http: connect to http://192.168.x.y:8080 timed out after 15s (server unreachable from this network?)
```
