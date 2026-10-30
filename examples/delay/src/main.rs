#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;

use ameba_hal::println;
use embassy_executor::Spawner;
use embassy_time::Instant;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    ameba_hal::init(ameba_hal::Config::default());

    test_blocking();
    test_async().await;
}

fn report(label: &str, elapsed: u64, lo: u64, hi: u64) {
    println!("{label} elapsed: {}ms", elapsed);
    if elapsed >= lo && elapsed <= hi {
        println!("[PASS]");
    } else {
        println!("[FAIL] expected {}-{}ms", lo, hi);
    }
}

fn test_blocking() {
    use embedded_hal::delay::DelayNs as _;
    let mut d = ameba_hal::delay::Delay::new();

    // delay_ms: direct measurement
    let t0 = Instant::now();
    d.delay_ms(50);
    report("[blocking] delay_ms(50)", t0.elapsed().as_millis(), 47, 55);

    // delay_ms > SYSTICK_RELOAD period (~70ms): exercises the slow path in spin_cycles.
    // 100ms = 24_000_000 cycles > RELOAD (16_777_215).
    let t0 = Instant::now();
    d.delay_ms(100);
    report(
        "[blocking] delay_ms(100) slow-path",
        t0.elapsed().as_millis(),
        97,
        105,
    );

    // delay_us: 100 × 1000µs = 100ms aggregated (1µs too short for 1kHz Instant)
    let t0 = Instant::now();
    for _ in 0..100 {
        d.delay_us(1_000);
    }
    report(
        "[blocking] delay_us(1000)*100",
        t0.elapsed().as_millis(),
        90,
        120,
    );

    // delay_ns ≥ 1µs: 100 × 100µs = 10ms aggregated
    let t0 = Instant::now();
    for _ in 0..100 {
        d.delay_ns(100_000);
    }
    report(
        "[blocking] delay_ns(100000)*100",
        t0.elapsed().as_millis(),
        9,
        13,
    );

    // delay_ns(500) = spin_cycles(120): 10000 × ~0.5µs = ~5ms
    let t0 = Instant::now();
    for _ in 0..10_000 {
        d.delay_ns(500);
    }
    report(
        "[blocking] delay_ns(500)*10000",
        t0.elapsed().as_millis(),
        4,
        12,
    );
}

async fn test_async() {
    use embedded_hal_async::delay::DelayNs as _;
    let mut d = ameba_hal::delay::Delay::new();

    // ≥ 1ms → Timer::after path (yields to executor)
    let t0 = Instant::now();
    d.delay_ms(50).await;
    report("[async] delay_ms(50)", t0.elapsed().as_millis(), 49, 55);

    // async delay_us(1000)*100 = ~100ms
    let t0 = Instant::now();
    for _ in 0..100 {
        d.delay_us(1_000).await;
    }
    report(
        "[async] delay_us(1000)*100",
        t0.elapsed().as_millis(),
        90,
        120,
    );

    // async delay_us(500)*1000 = ~500ms with 1MHz tick.
    // With old 1kHz tick each call rounds up to 1ms → ~1000ms (FAIL).
    let t0 = Instant::now();
    for _ in 0..1_000 {
        d.delay_us(500).await;
    }
    report(
        "[async] delay_us(500)*1000",
        t0.elapsed().as_millis(),
        450,
        600,
    );

    // async delay_ns characterization: embassy Timer always yields once (yielded_once flag),
    // so every call pays ~5µs executor overhead (2 inner-loop iterations + set_alarm peripheral
    // reads) regardless of the ns value. 500ns rounds to 1 tick (1µs) but the 1µs expires
    // before set_alarm even runs — no ISR fires; dequeue_expired wakes the task immediately.
    // Expected: 10000 × ~5µs = ~50ms. Tests that the async path works and overhead is stable.
    let t0 = Instant::now();
    for _ in 0..10_000 {
        d.delay_ns(500).await;
    }
    report(
        "[async] delay_ns(500)*10000 (executor-overhead dominated, ~5us/call)",
        t0.elapsed().as_millis(),
        40,
        70,
    );

    // async delay_ns(10000) = 10µs nominal + ~5µs overhead = ~15µs/call.
    // 1000 calls ≈ 15ms. Verifies that actual sleep time accumulates for larger ns values.
    let t0 = Instant::now();
    for _ in 0..1_000 {
        d.delay_ns(10_000).await;
    }
    report(
        "[async] delay_ns(10000)*1000",
        t0.elapsed().as_millis(),
        10,
        22,
    );
}
