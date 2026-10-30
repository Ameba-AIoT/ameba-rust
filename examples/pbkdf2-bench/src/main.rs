#![no_std]
#![no_main]

//! One-shot timing measurement for PBKDF2-SHA1-4096 (WPA2-PSK PMK derivation)
//! on the KM4TZ core @ 240MHz. Informs whether the PBKDF2 pass needs async
//! chunking to stay clear of the watchdog / other timing budgets when
//! connecting to a WPA2-PSK network.

use ameba_rt as _;
use cortex_m as _;

use ameba_hal::println;
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;

#[cortex_m_rt::entry]
fn main() -> ! {
    ameba_hal::init(ameba_hal::Config::default());

    let ssid = b"test-ssid-1234";
    let passphrase = b"correcthorsebatterystaple";
    let mut pmk = [0u8; 32];

    let t0 = ameba_hal::debugtimer::timestamp();
    pbkdf2_hmac::<Sha1>(passphrase, ssid, 4096, &mut pmk);
    let elapsed_us = ameba_hal::debugtimer::timestamp().wrapping_sub(t0);

    println!(
        "PBKDF2-SHA1-4096 elapsed: {} us ({} ms)",
        elapsed_us,
        elapsed_us / 1000
    );

    loop {
        cortex_m::asm::wfi();
    }
}
