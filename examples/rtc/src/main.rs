#![no_std]
#![no_main]

use ameba_hal::{
    println, rcc,
    rtc::{AmPm, ClockSource, Rtc, RtcConfig, TimeConfig, TimeFormat},
};
use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[RTC] start");
    ameba_hal::init(ameba_hal::Config::default());

    // Enable RTC clock in AON_CLK, then the function enable in AON_FEN.
    // Mirrors the C SDK sequence: RCC_PeriphClockCmd(NULL, RTC_CLOCK, EN)
    //   → RTC_Enable → RCC_PeriphClockCmd(APBPeriph_RTC, RTC_CLOCK, EN).
    rcc::enable(rcc::Peripheral::RTC);

    let rtc = unsafe { Rtc::steal() };
    rtc.enable(true);

    rcc::enable(rcc::Peripheral::RTC_FEN);

    let ok = rtc.init(&RtcConfig::default());
    if !ok {
        println!("[RTC] init failed");
        return;
    }

    // Set time to 2025-01-01 12:00:00
    let start_time = TimeConfig {
        am_pm: AmPm::Am,
        year: 2025,
        days: 1,
        hours: 12,
        minutes: 0,
        seconds: 0,
    };

    let ok = rtc.set_time(TimeFormat::Bin, &start_time);
    if !ok {
        println!("[RTC] set_time failed");
        return;
    }

    // Step 5 of the rtc/mod.rs usage sketch: switch off the default
    // Osc131kDiv4 source onto Sdm32k. Must come after `enable(true)` (fen_rtc
    // set) — SDK: "SDM32K clock source switching must be performed after
    // rtc_fen is enabled" (ameba_app_start.c : rtc_irq_init). Without this,
    // RTC free-runs on the uncalibrated ~32.75kHz OSC131K/4 tap instead of
    // the 32.768kHz crystal, and the diff-window check below would be loose.
    rtc.set_clock_source(ClockSource::Sdm32k);

    println!("[RTC] time set to 2025-01-01 12:00:00, waiting 3s...");

    let t0 = Instant::now();
    Timer::after(Duration::from_secs(3)).await;
    let elapsed_ms = t0.elapsed().as_millis();

    let t = rtc.get_time(TimeFormat::Bin);

    println!(
        "[RTC] read-back: 20{:02}-{:03} {:02}:{:02}:{:02}",
        (t.year - 2000) as u32,
        t.days as u32,
        t.hours as u32,
        t.minutes as u32,
        t.seconds as u32,
    );

    // seconds elapsed from RTC (compared to set point 12:00:00)
    let rtc_secs = t.hours as i32 * 3600 + t.minutes as i32 * 60 + t.seconds as i32;
    let set_secs = 12 * 3600;
    let diff = rtc_secs - set_secs;

    println!(
        "[RTC] diff: {}s  (embassy elapsed: {}ms)",
        diff, elapsed_ms as u32,
    );

    if !(2..=4).contains(&diff) {
        println!("[FAIL] RTC: diff out of expected window: {}s", diff);
    } else {
        println!("[PASS] RTC: tick OK");
    }
}
